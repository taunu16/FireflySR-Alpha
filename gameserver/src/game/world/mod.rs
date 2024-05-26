pub mod entity;
mod group;
use atomic_refcell::AtomicRef;
pub use group::SceneGroup;

use std::sync::atomic::{AtomicU32, Ordering};

use common::data::{level::{GroupLoadSide, LevelAnchorInfo, PropState}, PlaneType, LEVEL_TABLE};
use proto::{AvatarType, VectorBin};

use crate::net::PlayerSession;

use self::entity::{EntityType, SceneActor, SceneEntity, SceneMonster, SceneNpc, SceneProp};

use super::managers::ChallengeManager;

pub const INITIAL_PROP_STATE_OVERRIDES: [(u32, PropState); 1] = [
    // (103016, PropState::CheckPointEnable), //origami bird event
    // (102195, PropState::CheckPointEnable), //origami bird event
    // (2421, PropState::CheckPointEnable), //origami bird event
    // (2418, PropState::CheckPointEnable), //origami bird event
    // (2419, PropState::CheckPointEnable), //stellar shimmer event
    // (102195, PropState::CheckPointEnable), //stellar shimmer event
    (101, PropState::CheckPointEnable), //space anchors
];

#[derive(Default, Debug)]
pub struct GameWorld {
    pub entry_id: u32,
    pub plane_id: u32,
    pub floor_id: u32,
    pub leader_entity_id: u32,
    pub scene_group_list: Vec<SceneGroup>,
    pub plane_type: PlaneType,
    entity_counter: AtomicU32,
}

impl GameWorld {
    pub fn new(entry_id: u32, plane_id: u32, floor_id: u32, plane_type: PlaneType) -> Self {
        Self {
            entry_id,
            plane_id,
            floor_id,
            plane_type,
            leader_entity_id: 0,
            scene_group_list: Vec::new(),
            entity_counter: AtomicU32::new(0),
        }
    }

    pub fn is_actor(&self, entity_id: u32) -> bool {
        if let Some(group) = self.scene_group_list.iter().find(|g| g.id == 0) {
            group.actor_list.iter().any(|a| a.entity_id() == entity_id)
        } else {
            false
        }
    }

    pub fn add_player_actor(
        &mut self,
        uid: u32,
        avatar_id: u32,
        pos: Option<&VectorBin>,
        rot: Option<&VectorBin>,
    ) {
        let mut actor = SceneActor::new(
            EntityType::Actor as u32 * 1000000 + self.next_entity_id(),
            uid,
            avatar_id,
            AvatarType::AvatarFormalType.into(),
        );

        if let (Some(pos), Some(rot)) = (pos, rot) {
            actor.set_position(pos.x, pos.y, pos.z);
            actor.set_rotation(rot.x, rot.y, rot.z);
        } else {
            let anchor = self.get_player_start_position();
            actor.set_position(
                (anchor.pos_x * 1000.0) as i32,
                (anchor.pos_y * 1000.0) as i32,
                (anchor.pos_z * 1000.0) as i32,
            );
            actor.set_rotation(
                (anchor.rot_x * 1000.0) as i32,
                (anchor.rot_y * 1000.0) as i32,
                (anchor.rot_z * 1000.0) as i32,
            );
        }

        self.leader_entity_id = actor.entity_id();

        if let Some(group) = self.scene_group_list.iter_mut().find(|g| g.id == 0) {
            group.actor_list.push(actor);
        } else {
            self.scene_group_list.push(SceneGroup {
                id: 0,
                actor_list: vec![actor],
                prop_list: Vec::new(),
                npc_list: Vec::new(),
                monster_list: Vec::new(),
            });
        }
    }

    pub fn init_groups(&mut self, session: &PlayerSession, challenge_mgr: &AtomicRef<ChallengeManager>) {
        let data_bin = &session.player_info().data;

        let floor = LEVEL_TABLE.level_floors.get(&self.floor_id).unwrap();

        let mut stop_adding_entities = false;

        for group_instance in &floor.group_instance_list {
            let level_group = LEVEL_TABLE
                .get_level_group(self.floor_id, group_instance.id)
                .unwrap();

            if level_group.load_side == GroupLoadSide::Client {
                continue;
            }

            let mut prop_entity_list = Vec::new();
            if let Some(prop_list) = level_group.prop_list.as_ref() {
                for level_prop in prop_list {
                    let mut prop = SceneProp::new(
                        EntityType::Prop as u32 * 1000000 + self.next_entity_id(),
                        group_instance.id,
                        level_prop.id,
                        level_prop.prop_id,
                        level_prop.event_id.unwrap_or(0),
                        level_prop.mapping_info_id
                    );

                    prop.state = if let Some(val) = INITIAL_PROP_STATE_OVERRIDES.iter().find(|(i,_)| i == &prop.prop_id()) {
                        val.1.clone() as u32
                    } else {
                        level_prop.state.clone() as u32
                    };

                    if level_prop.init_level_graph.clone().unwrap_or_default().contains("_Door_") || level_group.group_name == Some("Doors".to_string()) {
                        prop.state = PropState::Open as u32;
                    }

                    if prop.prop_id() == 1003 { 
                        if level_prop.mapping_info_id == 2220 {
                            prop.state = PropState::Open as u32;
                        } else {
                            continue;
                        }
                    }

                    if [2418, 2421, 2206].contains(&level_prop.mapping_info_id) {
                        prop.state = PropState::Open as u32
                    }

                    'st: {
                        let Some(scenes) = &data_bin.scene_bin else {break 'st;};
                        let Some(scene) = scenes.scene_list.iter().find(|scene| scene.plane_id == self.plane_id && scene.floor_id == self.floor_id) else {break 'st;};
                        let Some(scene_prop) = scene.group_map.iter().map(|a|&a.1.prop_list).flatten().find(|p| p.entity_id == prop.entity_id()) else {break 'st;};

                        prop.state = scene_prop.state;
                    };
                    

                    prop.set_position(
                        (level_prop.pos_x * 1000.0) as i32,
                        (level_prop.pos_y * 1000.0) as i32,
                        (level_prop.pos_z * 1000.0) as i32,
                    );
                    prop.set_rotation(
                        (level_prop.rot_x * 1000.0) as i32,
                        (level_prop.rot_y * 1000.0) as i32,
                        (level_prop.rot_z * 1000.0) as i32,
                    );

                    prop_entity_list.push(prop);
                }
            }

            let mut npc_entity_list = Vec::new();
            if let Some(npc_list) = level_group.npc_list.as_ref() {
                for level_npc in npc_list {
                    if level_npc.is_client_only || level_npc.is_delete || (self.plane_id == 10000 && ![3, 40, 34].contains(&group_instance.id)) {
                        continue;
                    }

                    let mut npc = SceneNpc::new(
                        self.next_entity_id(), 
                        level_npc.id,
                        group_instance.id,
                        level_npc.npc_id
                    );

                    npc.set_position(
                        (level_npc.pos_x * 1000.0) as i32,
                        (level_npc.pos_y * 1000.0) as i32,
                        (level_npc.pos_z * 1000.0) as i32,
                    );
                    npc.set_rotation(
                        0,
                        (level_npc.rot_y * 1000.0) as i32,
                        0,
                    );

                    npc_entity_list.push(npc);
                }
            }

            let mut npc_monster_list = Vec::new();
            if let Some(monster_list) = level_group.monster_list.as_ref() {
                for level_monster in monster_list {
                    if stop_adding_entities {
                        break;
                    }
                    
                    if level_monster.is_client_only || level_monster.is_delete {
                        continue;
                    }

                    let mut monster = SceneMonster::new(
                        self.next_entity_id(), 
                        level_monster.id,
                        group_instance.id,
                        level_monster.npc_monster_id,
                        level_monster.event_id,
                        data_bin.basic_bin.as_ref().map(|a| a.world_level).unwrap_or(0)
                    );

                    
                    if challenge_mgr.is_challenge() {
                        challenge_mgr.load_monster(&mut monster);
                        stop_adding_entities = true;
                    }
                    

                    monster.set_position(
                        (level_monster.pos_x * 1000.0) as i32,
                        (level_monster.pos_y * 1000.0) as i32,
                        (level_monster.pos_z * 1000.0) as i32,
                    );
                    monster.set_rotation(
                        0,
                        (level_monster.rot_y * 1000.0) as i32,
                        0,
                    );

                    npc_monster_list.push(monster);
                }
            }

            if let Some(scene_group) = self.scene_group_list.iter_mut().find(|g| g.id == group_instance.id) {
                scene_group.prop_list.append(&mut prop_entity_list);
                scene_group.npc_list.append(&mut npc_entity_list);
            } else {
                self.scene_group_list.push(SceneGroup {
                    id: group_instance.id,
                    prop_list: prop_entity_list,
                    actor_list: Vec::new(),
                    npc_list: npc_entity_list,
                    monster_list: npc_monster_list
                });
            }
        }
    }

    fn get_player_start_position(&self) -> &LevelAnchorInfo {
        let floor = LEVEL_TABLE.level_floors.get(&self.floor_id).unwrap();
        let group_instance = &floor.group_instance_list[floor.start_group_index as usize];

        let group = LEVEL_TABLE
            .get_level_group(self.floor_id, group_instance.id)
            .unwrap();
        &group
            .anchor_list
            .as_ref()
            .unwrap()
            .iter()
            .find(|a| a.id == floor.start_anchor_id)
            .unwrap()
    }

    fn next_entity_id(&self) -> u32 {
        self.entity_counter.fetch_add(1, Ordering::SeqCst) + 1
    }
}
