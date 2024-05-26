use std::collections::HashMap;

use crate::{
    game::world::{entity::{SceneEntity, SceneMonster, SceneProp}, GameWorld}, net::PlayerSession, util
};

use super::*;
use atomic_refcell::AtomicRef;
use common::data::{EXCEL_COLLECTION, LEVEL_TABLE};
use proto::*;

pub struct SceneManager {
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
    world: AtomicRefCell<GameWorld>,
}

impl SceneManager {
    const MAX_SECTION_ID: u32 = 100;

    pub fn new(player_info: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self {
            player_info,
            world: AtomicRefCell::new(GameWorld::default()),
        }
    }

    pub fn init_defaults(&self) {
        let mut player_info = self.player_info.borrow_mut();

        player_info.data.scene_bin = Some(PlayerSceneCompBin {
            cur_entry_id: 2010101,
            cur_position: Some(MotionBin::default()),
            scene_list: Vec::new(),
        });
    }

    pub fn enter_scene(&mut self, session: &PlayerSession, entry_id: u32) -> Result<SceneInfo, Retcode> {
        self.load_scene(session, entry_id)?;
        Ok(self.cur_scene_info_proto())
    }

    pub async fn enter_scene_packet(&mut self, session: &PlayerSession, entry_id: u32, waypoint_id: Option<u32>) -> Result<SceneInfo, Retcode> {
        let scene_info = self.enter_scene(session, entry_id);

        if let Ok(scene_info) = scene_info.clone() {
            //let challenge_mgr = session.context.challenge_mgr.borrow();
            let lineup_mgr = session.context.lineup_mgr.borrow();

            let enter_scene_by_server = EnterSceneByServerScNotify {
                reason: EnterSceneReason::None.into(),
                lineup: Some(lineup_mgr.cur_lineup_proto()),
                scene: Some(scene_info.clone()),
            };

            // println!("{:#?}", enter_scene_by_server);

            let _ = session
                .send(CMD_ENTER_SCENE_BY_SERVER_SC_NOTIFY, enter_scene_by_server)
                .await;

            let mut anchor = Option::None;
            
            if let Some(floor) = LEVEL_TABLE.level_floors.get(&scene_info.clone().floor_id) {
                for group in floor.group_instance_list.iter().map(|gi| LEVEL_TABLE.get_level_group(scene_info.clone().floor_id, gi.id)).flatten() {
                    let _tmp = group.prop_list.clone().unwrap_or(vec![]);
                    
                    let Some(waypoint_id) = waypoint_id else {continue;};
                    let Some(prop) = _tmp.iter()
                        .find(|prop| prop.mapping_info_id == waypoint_id) else {continue;};

                    let Some(ancho) = group.anchor_list.as_ref().map(|gr| gr.iter().find(|anch|  prop.anchor_id == Some(anch.id))).flatten() else {continue;};
                    anchor = Some(ancho);
                    
                }
            }
            if anchor.is_some() {
                let _ = session.send(
                    CMD_SCENE_ENTITY_MOVE_SC_NOTIFY, 
                    SceneEntityMoveScNotify {
                        entity_id: 0, 
                        entry_id,
                        motion: Some(MotionInfo {
                            pos: Some(Vector {
                                x: (anchor.unwrap().pos_x * 1000.0) as i32,
                                y: (anchor.unwrap().pos_y * 1000.0) as i32,
                                z: (anchor.unwrap().pos_z * 1000.0) as i32,
                            }),
                            rot: Some(Vector {
                                y: (anchor.unwrap().rot_y * 1000.0) as i32,
                                ..Default::default()
                            })
                        }),
                        ..Default::default()
                    }).await;
            }

            let _ = lineup_mgr.sync_cur_lineup(session).await;
        }

        scene_info
    }

    pub fn entity_move(&mut self, entity_motion: &EntityMotion, entry_id: u32) {
        let mut player_info = self.player_info.borrow_mut();
        let scene_comp = player_info.data.scene_bin.as_mut().unwrap();

        if entity_motion.entity_id <= 1000000 {
            return;
        }

        let motion = entity_motion.motion.as_ref().unwrap();
        let pos = motion.pos.as_ref().unwrap();
        let rot = motion.rot.as_ref().unwrap();

        scene_comp.cur_position = Some(MotionBin {
            pos: Some(VectorBin {
                x: pos.x,
                y: pos.y,
                z: pos.z,
            }),
            rot: Some(VectorBin {
                x: rot.x,
                y: rot.y,
                z: rot.z,
            }),
        });
        scene_comp.cur_entry_id = entry_id;
    }

    pub fn refresh_actor_group(&mut self, lineup_mgr: AtomicRef<LineupManager>) -> Option<SceneGroupRefreshInfo> {
        let player_uid = self.player_info.borrow().uid;

        self.world
            .borrow_mut()
            .scene_group_list
            .retain(|g| g.id != 0);

        self.cur_lineup_avatars(lineup_mgr).iter().for_each(|id| {
            self.world
                .borrow_mut()
                .add_player_actor(player_uid, *id, None, None)
        });

        let world = self.world.borrow();
        world
            .scene_group_list
            .iter()
            .find(|g| g.id == 0)
            .map(|g| SceneGroupRefreshInfo {
                group_id: g.id,
                state: 1,
                refresh_type: SceneGroupRefreshType::None.into(),
                refresh_entity_list: g
                    .actor_list
                    .iter()
                    .map(|a| SceneGroupEntityRefreshInfo {
                        refresh_entity_info: Some(a.scene_entity_info_proto()),
                        ..Default::default()
                    })
                    .collect(),
            })
    }

    pub fn get_prop(&self, entity_id: u32) -> Option<(u32, SceneProp)> {
        for group in &self.world.borrow().scene_group_list {
            for prop in &group.prop_list {
                if prop.entity_id() == entity_id {
                    return Some((group.id, prop.clone()));
                }
            }
        }
        None
    }

    pub fn get_monster(&self, entity_id: u32) -> Option<(u32, SceneMonster)> {
        for group in &self.world.borrow().scene_group_list {
            for monster in &group.monster_list {
                if monster.entity_id() == entity_id {
                    return Some((group.id, monster.clone()));
                }
            }
        }
        None
    }

    pub fn set_prop_state(&self, group_id: u32, entity_id: u32, state: u32) {
        let mut world = self.world.borrow_mut();
        let Some(group) = world.scene_group_list.iter_mut().find(|a| a.id == group_id) else {return;};
        let Some(prop) = group.prop_list.iter_mut().find(|p| p.entity_id() == entity_id) else {return;};

        prop.state = state;
        
        let Some(scenes) = &mut self.player_info.borrow_mut().data.scene_bin else {return;};
        let Some(scene) = scenes.scene_list.iter_mut().find(|scene| scene.plane_id == world.plane_id && scene.floor_id == world.floor_id) else {return;};
        let Some(scene_group) = scene.group_map.get_mut(&group_id) else {
            scene.group_map.insert(group_id, GroupBin {
                prop_list: vec![
                    PropBin {
                        entity_id,
                        state
                    }
                ]
            });
            return;
        };
        let Some(scene_prop) = scene_group.prop_list.iter_mut().find(|p| p.entity_id == entity_id) else {
            scene_group.prop_list.push(PropBin {
                entity_id,
                state
            });
            return;
        };

        scene_prop.state = state;
    }

    pub fn reset_prop_states(&self) {
        let Some(scenes) = &mut self.player_info.borrow_mut().data.scene_bin else {return;};
        
        for scene in scenes.scene_list.iter_mut() {
            scene.group_map = HashMap::new();
        }
    }

    // pub fn get_prop_state(&self, entity_id: u32) -> u32 {
    //     let mut world = self.world.borrow_mut();
    //     macro_rules! from_world {
    //         () => {{
    //             let Some(prop) = world.scene_group_list.iter_mut().map(|a| &mut a.prop_list).flatten().find(|p| p.entity_id() == entity_id) else {return PropState::Closed as u32;};

    //             return prop.state;
    //         }};
    //     }
        
    //     let Some(scenes) = &mut self.player_info.borrow_mut().data.scene_bin else {from_world!();};
    //     let Some(scene) = scenes.scene_list.iter_mut().find(|scene| scene.plane_id == world.plane_id && scene.floor_id == world.floor_id) else {from_world!();};
    //     let Some(scene_prop) = scene.group_map.iter_mut().map(|a|&mut a.1.prop_list).flatten().find(|p| p.entity_id == entity_id) else {from_world!()};

    //     scene_prop.state
    // }

    fn load_scene(&mut self, session: &PlayerSession, entry_id: u32) -> Result<(), Retcode> {
        let challenge_mgr = session.context.challenge_mgr.borrow();
        let cur_avatar_id_list = self.cur_lineup_avatars(session.context.lineup_mgr.borrow());

        let mut player_info = self.player_info.borrow_mut();
        let player_uid = player_info.uid;
        let scene_comp = player_info.data.scene_bin.as_mut().unwrap();

        let Some(entrance_config) = EXCEL_COLLECTION
            .map_entrance_configs
            .iter()
            .find(|c| c.id == entry_id)
        else {
            return Err(Retcode::RetAdventureMapNotExist);
        };

        let Some(plane_config) = EXCEL_COLLECTION
            .maze_plane_configs
            .iter()
            .find(|c| c.plane_id == entrance_config.plane_id)
        else {
            return Err(Retcode::RetAdventureMapNotExist);
        };

        let mut world =
            GameWorld::new(entry_id, entrance_config.plane_id, entrance_config.floor_id, plane_config.plane_type.clone());

        let (pos, rot) = if scene_comp.cur_entry_id == entry_id {
            let motion = scene_comp.cur_position.as_ref().unwrap();
            (motion.pos.clone(), motion.rot.clone())
        } else {
            (None, None)
        };

        cur_avatar_id_list
            .iter()
            .for_each(|id| world.add_player_actor(player_uid, *id, pos.as_ref(), rot.as_ref()));
        
        drop(player_info); //i have ran out of ideas 

        world.init_groups(session, &challenge_mgr);

        let mut player_info = self.player_info.borrow_mut();
        let scene_comp = player_info.data.scene_bin.as_mut().unwrap();
        
        self.world = AtomicRefCell::new(world);

        if !challenge_mgr.is_challenge() {
            scene_comp.cur_entry_id = entry_id;
        }


        if let Some(scene_bin) = scene_comp.scene_list.iter_mut().find(|s| {
            s.plane_id == entrance_config.plane_id && s.floor_id == entrance_config.floor_id
        }) {
            scene_bin.last_enter_time = util::cur_timestamp_seconds() as i64;
        } else {
            scene_comp.scene_list.push(SceneBin {
                plane_id: entrance_config.plane_id,
                floor_id: entrance_config.floor_id,
                lighten_section_list: Vec::new(),
                unlocked_teleport_list: Vec::new(),
                last_enter_time: util::cur_timestamp_seconds() as i64,
                group_map: HashMap::new()
            });
        }

        Ok(())
    }

    pub fn enter_section(&self, section_id: u32) {
        if section_id < Self::MAX_SECTION_ID {
            let world = self.world.borrow();

            let mut player_info = self.player_info.borrow_mut();
            let scene_comp = player_info.data.scene_bin.as_mut().unwrap();

            if let Some(scene_bin) = scene_comp
                .scene_list
                .iter_mut()
                .find(|s| s.plane_id == world.plane_id && s.floor_id == world.floor_id)
            {
                if !scene_bin.lighten_section_list.contains(&section_id) {
                    scene_bin.lighten_section_list.push(section_id);
                }
            }
        }
    }

    fn world_to_proto(&self, world: AtomicRef<GameWorld>) -> SceneInfo {
        let player_info = self.player_info.borrow();
        let scene_comp = player_info.data.scene_bin.as_ref().unwrap();
        let scene_bin = scene_comp
            .scene_list
            .iter()
            .find(|s| s.plane_id == world.plane_id && s.floor_id == world.floor_id)
            .unwrap();

        SceneInfo {
            plane_id: world.plane_id,
            floor_id: world.floor_id,
            entry_id: world.entry_id,
            game_mode_type: world.plane_type.clone() as u32,
            leader_entity_id: world.leader_entity_id,
            scene_group_list: world.scene_group_list.iter().map(|g| g.info()).collect(),
            lighten_section_list: scene_bin.lighten_section_list.clone(),
            ..Default::default()
        }
    }

    pub fn cur_scene_info_proto(&self) -> SceneInfo {
        let world = self.world.borrow();

        self.world_to_proto(world)
    }

    pub fn cur_entry_id(&self) -> u32 {
        let player_info = self.player_info.borrow();
        let scene_comp = player_info.data.scene_bin.as_ref().unwrap();

        scene_comp.cur_entry_id
    }

    fn cur_lineup_avatars(&self, lineup_mgr: AtomicRef<LineupManager>) -> Vec<u32> {
        // let player_info = self.player_info.borrow();
        // let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

        // let cur_lineup = lineup_comp
        //     .lineup_list
        //     .iter()
        //     .find(|l| l.index == lineup_comp.cur_lineup_index)
        //     .unwrap();
        // cur_lineup.avatar_list.iter().map(|a| a.avatar_id).collect()
        let mut avatars = lineup_mgr.cur_lineup_proto().avatar_list.clone();
        avatars.sort_by(|a, b| a.slot.partial_cmp(&b.slot).unwrap());
        avatars.iter().map(|a| a.id).collect()
    }
}
