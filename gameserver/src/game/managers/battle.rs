use std::collections::HashMap;
use atomic_refcell::AtomicRef;
use common::data::{StageType, EXCEL_COLLECTION};
use paste::paste;
use proto::{AssistMonsterWave, BattleBuff, BattleTarget, BattleTargetList, Kjfnknacfin, LineupAvatarBin, SceneBattleInfo, SceneMonsterData, SceneMonsterWave};
use rand::RngCore;

use crate::{game::world::entity::SceneMonster, net::PlayerSession};

use super::*;

macro_rules! listify_monsters {
    ($monster:ident, $($num:pat),*) => {
        paste! {
            vec![
                $(
                    SceneMonsterData {
                        monster_id: $monster.[<monster$num>],
                        ..Default::default()
                    }
                ),*
            ].into_iter().filter(|a| a.monster_id > 0).collect()
        }
    };
}

pub struct BattleManager {
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
}

impl BattleManager {
    pub fn new(player_info: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self { player_info }
    }

    pub fn init_defaults(&self) {
        //! TEMPORARY SOLUTION UNITL IT SYNC WITH CLIENT
        //todo: sync with client
        if let Some(lineup_bin) = self.player_info.borrow_mut().data.lineup_bin.as_mut() {
            lineup_bin.buffs.clear();
        }
    }

    fn buffs_reset(&self) -> Vec<BattleBuff> {
        let mut binding = self.player_info.borrow_mut();
        let lineup_bin = binding.data.lineup_bin.as_mut().unwrap();
        let ret = lineup_bin.buffs.clone().into_iter().map(|buff| BattleBuff {
            id: buff.buff_id,
            owner_index: buff.slot,
            level: 1,
            wave_flag: 0xffffff,
            ..Default::default()
        }).collect();
        lineup_bin.buffs.clear();
        ret
    }

    fn cur_lineup(&self, avatar_mgr: &AtomicRef<AvatarManager>) -> Vec<proto::BattleAvatar> {
        let binding = self.player_info.borrow();
        let lineup_comp = binding.data.lineup_bin.as_ref().unwrap();

        avatar_mgr.avatar_list_battle_proto(lineup_comp.lineup_list
        .iter()
        .find(|l| l.index == lineup_comp.cur_lineup_index)
        .unwrap().clone().avatar_list)
    }

    fn get_stage_waves(stage_id: u32) -> Vec<SceneMonsterWave> {
        let Some(stage) = EXCEL_COLLECTION.stage_configs.iter().find(|s| s.stage_id == stage_id) else {return vec![];};
        stage.monster_list.iter().map(|m: &common::data::MonsterList| 
            SceneMonsterWave {
                monster_list: listify_monsters!(m, 0, 1, 2, 3, 4),
                acpannfhach: Some(Kjfnknacfin {
                    level: stage.level,
                    ..Default::default()
                }),
                ..Default::default()
            }
        ).collect()
    }

    pub fn start_battle(&self, session: &PlayerSession, assist_waves: Vec<AssistMonsterWave>, hit_monsters: Vec<u32>) -> SceneBattleInfo {
        let avatar_mgr = session.context.avatar_mgr.borrow();
        let scene_mgr = session.context.scene_mgr.borrow();
        let challenge_mgr = session.context.challenge_mgr.borrow();

        let mut battle = SceneBattleInfo {
            stage_id: 201012311,
            logic_random_seed: rand::thread_rng().next_u32() % 1000000,
            battle_id: 1,
            battle_avatar_list: self.cur_lineup(&avatar_mgr),
            buff_list: self.buffs_reset(),
            ..Default::default()
        };

        let mut monsters = vec![];

        for entity_id in hit_monsters {
            let Some((_, monster)) = scene_mgr.get_monster(entity_id) else {continue;};
            monsters.push(monster);
        }

        for monster in &monsters {
            let Some(stage) = EXCEL_COLLECTION.stage_configs.iter().find(|s| s.stage_id == monster.stage_id) else {continue;};

            if stage.stage_type == StageType::Challenge {
                return self.start_challenge_battle(avatar_mgr, challenge_mgr, monsters);
            }

            battle.monster_wave_list.append(&mut stage.monster_list.iter().map(|v| SceneMonsterWave {
                monster_list: listify_monsters!(v, 0, 1, 2, 3, 4),
                acpannfhach: Some(Kjfnknacfin {
                    level: stage.level,
                    ..Default::default()
                }),
                ..Default::default()
            }).collect())
        }

        for (i, assist_wave) in assist_waves.iter().enumerate() {
            if let Some(wave) = battle.monster_wave_list.get_mut(i) {
                for entity_id in &assist_wave.entity_id_list {
                    let Some((_, monster)) = scene_mgr.get_monster(*entity_id) else {continue;};
                    wave.monster_list.push(SceneMonsterData {
                        monster_id: monster.monster_id,
                        ..Default::default()
                    })
                }
            } else {
                let mut wave = SceneMonsterWave::default();

                for entity_id in &assist_wave.entity_id_list {
                    let Some((_, monster)) = scene_mgr.get_monster(*entity_id) else {continue;};
                    wave.monster_list.push(SceneMonsterData {
                        monster_id: monster.monster_id,
                        ..Default::default()
                    })
                }

                battle.monster_wave_list.push(wave);
            }
        }

        battle
    }

    pub fn start_stage_battle(&self, avatar_mgr: AtomicRef<AvatarManager>, stage_id: u32) -> SceneBattleInfo {
        SceneBattleInfo {
            battle_avatar_list: self.cur_lineup(&avatar_mgr),
            monster_wave_list: BattleManager::get_stage_waves(stage_id),
            stage_id,
            logic_random_seed: rand::thread_rng().next_u32() % 1000000,
            battle_id: 1,
            buff_list: self.buffs_reset(),
            ..Default::default()
        }
    }

    pub fn start_challenge_battle(&self, avatar_mgr: AtomicRef<AvatarManager>, challenge_mgr: AtomicRef<ChallengeManager>, monsters: Vec<SceneMonster>) -> SceneBattleInfo {
        let challenge = challenge_mgr.get_challenge();
        // let score = challenge.score1 + challenge.score2;
        // let is_apoc_shadow = challenge.config.group_id == 3001;

        SceneBattleInfo {
            // cleheggdkal: 1,
            ohfkoaahoib: challenge.turns,
            battle_avatar_list: avatar_mgr.avatar_list_battle_proto(if challenge.stage == 1 {challenge.lineup_1.clone()} else {challenge.lineup_2.clone()}
                .iter().enumerate().map(|(i, id)| LineupAvatarBin {
                    avatar_id: *id,
                    avatar_type: 0,
                    hp: 10000,
                    slot: i as u32,
                    sp: 10000/2
                }).collect()),
            monster_wave_list: BattleManager::get_stage_waves(monsters[0].stage_id), //monsters.iter().map(|mon| BattleManager::get_stage_waves(mon.stage_id)).flatten().collect(),
            battle_id: 1,
            stage_id: monsters[0].stage_id,
            logic_random_seed: rand::thread_rng().next_u32() % 1000000,
            buff_list: self.buffs_reset(),
            battle_target_info: HashMap::from([
                (1, BattleTargetList {
                    battle_target_list: vec![BattleTarget {
                        id: 10001,
                        progress: challenge.score1,//if is_apoc_shadow { 2000 - score } else { score },
                        ..Default::default()
                    }, BattleTarget {
                        id: 10002,
                        progress: challenge.score2,//if is_apoc_shadow { 2000 - score } else { score },
                        ..Default::default()
                    }]
                }),
                // (5, BattleTargetList {
                //     battle_target_list: challenge.config.challenge_target_id.iter().map(|id| BattleTarget {
                //         id: *id,
                //         progress: challenge.score1 + challenge.score2,
                //         ..Default::default()
                //     }).collect()
                // })
            ]),
            ..Default::default()
        }
    }
}