use anyhow::Ok;
use atomic_refcell::AtomicRef;
use common::data::{ChallengeMazeConfig, EXCEL_COLLECTION};
use proto::{AmountInfo, AvatarType, BattleEndStatus, Bkamlmppdgl, ExtraLineupType, ItemList, LineupAvatar, LineupInfo, MotionInfo, Nmlilpfiidn, PveBattleResultCsReq, PveBattleResultScRsp, Retcode, SceneEntityMoveScNotify, Vector, CMD_CHALLENGE_SETTLE_NOTIFY, CMD_LEAVE_CHALLENGE_SC_RSP, CMD_P_V_E_BATTLE_RESULT_SC_RSP, CMD_QUIT_BATTLE_SC_NOTIFY, CMD_SCENE_ENTITY_MOVE_SC_NOTIFY};
use paste::paste;
use crate::{game::world::entity::SceneMonster, net::PlayerSession};

use super::*;

macro_rules! match_to_stage {
    ($challenge: expr, $prop: expr) => {
        paste! {
            match $challenge.stage {
                1 => $challenge.config.[<$prop 1>].clone(),
                _ => $challenge.config.[<$prop 2>].clone(),
            }
        }
    };
}

pub struct Challenge {
    pub lineup_1: Vec<u32>,
    pub lineup_2: Vec<u32>,
    pub stage: u32,
    pub turns: u32,
    pub score1: u32,
    pub score2: u32,
    pub id: u32,
    pub config: &'static ChallengeMazeConfig
}

pub struct ChallengeManager {
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
    active_challenge: Option<AtomicRefCell<Challenge>>
}

impl ChallengeManager {
    pub fn new(player_info: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self { player_info, active_challenge: Option::None }
    }

    pub fn init_defaults(&self) {
        
    }

    pub fn vec_to_lineup(ids: Vec<u32>, typ: ExtraLineupType) -> LineupInfo {
        LineupInfo {
            avatar_list: ids.iter().enumerate().map(|(i, id)| LineupAvatar {
                id: *id,
                avatar_type: AvatarType::AvatarFormalType as i32,
                slot: i as u32,
                hp: 10000,
                sp: Some(AmountInfo {
                    cur_amount: 10000/2,
                    max_amount: 10000
                }),
                ..Default::default()
            }).collect(),
            mp: 5,
            mp_max: 5,
            leader_slot: 0,
            extra_lineup_type: typ as i32,
            index: 0,
            name: String::new(),
            is_virtual: false,
            ..Default::default()
        }
    }

    pub fn cur_challenge_lineup(&self) -> Vec<u32> {
        let challenge = self.active_challenge.as_ref().unwrap().borrow();
        if challenge.stage == 1 {
            challenge.lineup_1.clone()
        } else {
            challenge.lineup_2.clone()
        }
    }

    pub fn cur_challenge_lineup_proto(&self, plane_id: u32) -> LineupInfo {
        let mut lineup = Self::vec_to_lineup(self.cur_challenge_lineup(), ExtraLineupType::LineupNone);
        lineup.plane_id = plane_id;
        lineup
    }

    pub fn start_challenge(&mut self, challenge_id: u32, lineup_1: Vec<u32>, lineup_2: Vec<u32>) -> Result<(&ChallengeMazeConfig, Vec<LineupInfo>), Retcode> {
        let challenge = match EXCEL_COLLECTION.challenge_maze_configs.iter().find(|mc| mc.id == challenge_id) {
            Some(v) => v,
            Option::None => return Err(Retcode::RetChallengeNotExist)
        };

        let mut act_challenge = Challenge {
            lineup_1: lineup_1.clone(),
            lineup_2: lineup_2.clone(),
            stage: 1,
            turns: challenge.challenge_count_down,
            score1: 0,
            score2: 0,
            id: challenge_id,
            config: challenge
        };

        if let Some(challenge_story_extra) = EXCEL_COLLECTION.challenge_story_extra_configs.iter().find(|a| a.id == challenge_id) {
            act_challenge.turns = challenge_story_extra.turn_limit;
        }

        self.active_challenge = Some(AtomicRefCell::new(act_challenge));

        Result::Ok((challenge, vec![ChallengeManager::vec_to_lineup(lineup_1, ExtraLineupType::LineupChallenge), ChallengeManager::vec_to_lineup(lineup_2, ExtraLineupType::LineupChallenge2)]))
    }

    pub fn is_challenge(&self) -> bool {
        self.active_challenge.is_some()
    }

    pub fn get_challenge(&self) -> AtomicRef<Challenge> {
        self.active_challenge.as_ref().unwrap().borrow()
    }

    pub async fn leave_challenge(session: &PlayerSession) -> anyhow::Result<()> {
        let this = session.context.challenge_mgr.borrow_mut();
        let scene_mgr = &mut session.context.scene_mgr.borrow_mut();
        let lineup_mgr = session.context.lineup_mgr.borrow();

        lineup_mgr.set_lineup_type(ExtraLineupType::LineupNone);

        let player_info = this.player_info.borrow();
        let scene_bin = player_info.data.scene_bin.as_ref().unwrap();
    
        let cur_entry_id = scene_bin.cur_entry_id;

        drop(player_info);
        drop(this);

        if let Result::Ok(scene_info) = scene_mgr.enter_scene_packet(session, cur_entry_id, Option::None).await {
            let player_info = session.player_info();
            let scene_bin = player_info.data.scene_bin.as_ref().unwrap();

            if let Some(motion_bin) = &scene_bin.cur_position {
                let _ = session.send(
                    CMD_SCENE_ENTITY_MOVE_SC_NOTIFY, 
                    SceneEntityMoveScNotify {
                        entity_id: 0,
                        entry_id: scene_info.entry_id,
                        motion: Some(
                            MotionInfo {
                                pos: motion_bin.pos.clone().map(|a| Vector{x:a.x,y:a.y,z:a.z}),
                                rot: motion_bin.rot.clone().map(|a| Vector{x:a.x,y:a.y,z:a.z})
                            }
                        ),
                        ..Default::default()
                    }
                ).await;
            }
        }

        let mut this = session.context.challenge_mgr.borrow_mut(); //i've REALLY ran out of ideas
        this.active_challenge = Option::None;

        let _ = session.send_dummy(CMD_QUIT_BATTLE_SC_NOTIFY).await;
        session.send_dummy(CMD_LEAVE_CHALLENGE_SC_RSP).await
    }

    pub fn load_monster(&self, monster: &mut SceneMonster) {
        let challenge = self.active_challenge.as_ref().unwrap().borrow();
        
        let config_list = match_to_stage!(challenge, config_list);
        let Some(monster_i) = config_list.iter().position(|a| a == &monster.inst_id()) else {return;};

        monster.monster_id = match_to_stage!(challenge, npc_monster_id_list)[monster_i];
        monster.event_id = match_to_stage!(challenge, event_id_list)[monster_i];
        monster.stage_id = monster.event_id;
    }

    async fn finish_challenge(session: &PlayerSession, won: bool) -> anyhow::Result<()> {
        let mut this = session.context.challenge_mgr.borrow_mut();
        let challenge = this.active_challenge.as_mut().unwrap().borrow();
        let lineup_mgr = session.context.lineup_mgr.borrow();
        lineup_mgr.set_lineup_type(ExtraLineupType::LineupNone);

        if !won {
            session.send(
                CMD_CHALLENGE_SETTLE_NOTIFY, 
                Bkamlmppdgl {
                    bmikmflhmjd: false,
                    challenge_id: challenge.id,
                    ..Default::default()
                }
            ).await?;

            drop(challenge);
            this.active_challenge = Option::None;
            return Ok(())
        }

        Ok(())
    }

    pub async fn battle_result(session: &PlayerSession, body: &PveBattleResultCsReq) -> anyhow::Result<()> {println!("{:?}", body);
        session.send(
            CMD_P_V_E_BATTLE_RESULT_SC_RSP, 
            PveBattleResultScRsp {
                battle_id: body.battle_id,
                retcode: 0,
                stage_id: body.stage_id,
                end_status: body.end_status,
                affnalechoi: body.res_version.to_string(),
                njaeinhfbem: String::new(),
                dehnokcapmd: true,
                ..Default::default()
            }
        ).await?;

        match body.end_status() {
            BattleEndStatus::BattleEndWin => ChallengeManager::advance_stage(session).await,
            BattleEndStatus::BattleEndLose => ChallengeManager::finish_challenge(session, false).await,
            _ => Ok(())
        }
    }

    async fn advance_stage(session: &PlayerSession) -> anyhow::Result<()> {
        let mut this = session.context.challenge_mgr.borrow_mut();
        let mut scene_mgr = session.context.scene_mgr.borrow_mut();
        let mut challenge = this.active_challenge.as_mut().unwrap().borrow_mut();
        let lineup_mgr = session.context.lineup_mgr.borrow();

        if challenge.stage >= challenge.config.stage_num {
            let challenge_id = challenge.id;
            lineup_mgr.set_lineup_type(ExtraLineupType::LineupNone);
            drop(challenge);
            this.active_challenge = Option::None;
            session.send(
                CMD_CHALLENGE_SETTLE_NOTIFY, 
                Bkamlmppdgl {
                    bmikmflhmjd: true,
                    challenge_id,
                    fckicpefihi: 2100, //score one
                    ijjfhfincfd: 2,
                    hlfjjlgkjci: 37, //score two
                    knipkmeghia: 3,
                    reward: Some(ItemList {item_list: vec![]}),
                    jjbhoplpacc: Some(Nmlilpfiidn {
                        level: 3,
                        pafenhngbpi: 6 //stars
                    }),
                    ..Default::default()
                }
            ).await
        } else {
            lineup_mgr.set_lineup_type(ExtraLineupType::LineupChallenge2);
            let _ = lineup_mgr.sync_cur_lineup(session).await;
            challenge.stage += 1;

            let entry_id = challenge.config.map_entrance_id2;

            drop(challenge);
            drop(this); //lack of dropping references hurt

            let _ = scene_mgr.enter_scene_packet(session, entry_id, Option::None).await;
            Ok(())
        }
    }

    // pub async fn skill_cast(&self, session: &PlayerSession, body: &SceneCastSkillCsReq) -> anyhow::Result<()> {
    //     if !(body.caster_id > 1000000 && body.hit_target_entity_id_list.len() > 0) {
    //         return session.send(
    //             CMD_SCENE_CAST_SKILL_SC_RSP,
    //             SceneCastSkillScRsp {
    //                 attacked_group_id: body.attacked_group_id,
    //                 retcode: 0,
    //                 ..Default::default()
    //             } 
    //         ).await
    //     }

    //     let challenge = self.active_challenge.as_ref().unwrap().borrow();
    //     let avatar_mgr = session.context.avatar_mgr.borrow();

    //     let battle = SceneBattleInfo {
    //         //cleheggdkal: 10, //wave count
    //         ohfkoaahoib: challenge.turns,
    //         stage_id: match_to_stage!(challenge, event_id_list)[0],
    //         logic_random_seed: rand::thread_rng().next_u32() % 1000000,
    //         battle_id: 1,
    //         battle_avatar_list: avatar_mgr.avatar_list_battle_proto(if challenge.stage == 1 {challenge.lineup_1.clone()} else {challenge.lineup_2.clone()}.iter().enumerate().map(|(i, id)| LineupAvatarBin {
    //             avatar_id: *id,
    //             avatar_type: 0,
    //             hp: 10000,
    //             slot: i as u32,
    //             sp: 10000/2
    //         }).collect()),
    //         monster_wave_list: match_to_stage!(challenge, npc_monster_id_list).iter().map(|id| SceneMonsterWave {
    //             monster_list: vec![
    //                 SceneMonsterData {
    //                     monster_id: *id,
    //                     ..Default::default()
    //                 }
    //             ],
    //             ..Default::default()
    //         }).collect(),
    //         ..Default::default()
    //     };


    //     session.send(
    //         CMD_SCENE_CAST_SKILL_SC_RSP, 
    //         SceneCastSkillScRsp {
    //             attacked_group_id: body.attacked_group_id,
    //             retcode: 0,
    //             battle_info: Some(battle),
    //             ..Default::default()
    //         }
    //     ).await
    // }
}