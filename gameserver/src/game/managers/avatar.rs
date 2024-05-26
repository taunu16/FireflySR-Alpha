use std::collections::HashMap;

use super::*;
use anyhow::anyhow;
use common::data::EXCEL_COLLECTION;
use proto::*;

pub struct AvatarManager {
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
    item_mgr: Weak<AtomicRefCell<ItemManager>>,
}

impl AvatarManager {
    const HERO_AVATAR_ID: u32 = 8001;

    pub fn new(
        player_info: Arc<AtomicRefCell<PlayerInfo>>,
        item_mgr: Arc<AtomicRefCell<ItemManager>>,
    ) -> Self {
        Self {
            player_info,
            item_mgr: Arc::downgrade(&item_mgr),
        }
    }

    pub fn init_defaults(&self) {
        self.create_comp();
        let hero_basic_type = self.player_info.borrow().get_cur_basic_type() as u32;

        self.unlock_avatar(hero_basic_type).unwrap();
        self.unlock_all_avatars();
    }

    pub fn has_avatar(&self, avatar_id: u32) -> bool {
        let avatar_id = Self::base_avatar_id(avatar_id);

        let player_info = self.player_info.borrow();
        let avatar_comp = player_info.data.avatar_bin.as_ref().unwrap();

        avatar_comp
            .avatar_list
            .iter()
            .any(|a| a.avatar_id == avatar_id)
    }

    pub fn dress_equipment(&self, avatar_id: u32, equipment_uid: u32) -> Result<(), Retcode> {
        if !self.has_avatar(avatar_id) {
            return Err(Retcode::RetAvatarNotExist);
        }

        let item_mgr = self.item_mgr.upgrade().unwrap();
        let item_mgr = item_mgr.borrow();
        item_mgr.assign_equipment(equipment_uid, Self::base_avatar_id(avatar_id))?;

        let prev_equipment_uid = self.set_equipment_uid(avatar_id, equipment_uid)?;
        if prev_equipment_uid != 0 {
            let _ = item_mgr.assign_equipment(prev_equipment_uid, 0);
        }

        Ok(())
    }

    pub fn dress_relic(
        &self,
        avatar_id: u32,
        relic_uid: u32,
        relic_slot: u32,
    ) -> Result<(), Retcode> {
        if !self.has_avatar(avatar_id) {
            return Err(Retcode::RetAvatarNotExist);
        }

        let item_mgr = self.item_mgr.upgrade().unwrap();
        let item_mgr = item_mgr.borrow();
        item_mgr.assign_relic(relic_uid, Self::base_avatar_id(avatar_id))?;

        let prev_relic_uid = self.set_relic_uid(avatar_id, relic_uid, relic_slot)?;
        if prev_relic_uid != 0 {
            let _ = item_mgr.assign_relic(prev_relic_uid, 0);
        }

        Ok(())
    }

    pub fn take_off_relic(&self, avatar_id: u32, relic_slot: u32) -> Result<(), Retcode> {
        let relic_uid = self.set_relic_uid(Self::base_avatar_id(avatar_id), 0, relic_slot)?;

        let item_mgr = self.item_mgr.upgrade().unwrap();
        let item_mgr = item_mgr.borrow();
        let _ = item_mgr.assign_relic(relic_uid, 0);

        Ok(())
    }

    fn set_equipment_uid(&self, avatar_id: u32, equipment_uid: u32) -> Result<u32, Retcode> {
        let avatar_id = Self::base_avatar_id(avatar_id);

        let mut player_info = self.player_info.borrow_mut();
        let avatar_comp = player_info.data.avatar_bin.as_mut().unwrap();

        let avatar = avatar_comp
            .avatar_list
            .iter_mut()
            .find(|a| a.avatar_id == avatar_id)
            .ok_or(Retcode::RetAvatarNotExist)?;

        let prev_equipment_uid = avatar.equipment_unique_id;
        avatar.equipment_unique_id = equipment_uid;

        Ok(prev_equipment_uid)
    }

    fn set_relic_uid(
        &self,
        avatar_id: u32,
        relic_uid: u32,
        relic_slot: u32,
    ) -> Result<u32, Retcode> {
        let avatar_id = Self::base_avatar_id(avatar_id);

        let mut player_info = self.player_info.borrow_mut();
        let avatar_comp = player_info.data.avatar_bin.as_mut().unwrap();

        let avatar = avatar_comp
            .avatar_list
            .iter_mut()
            .find(|a| a.avatar_id == avatar_id)
            .ok_or(Retcode::RetAvatarNotExist)?;

        let prev_relic_uid = avatar
            .relic_map
            .get(&relic_slot)
            .map(|i| *i)
            .unwrap_or_default();

        *avatar.relic_map.entry(relic_slot).or_insert(relic_uid) = relic_uid;
        Ok(prev_relic_uid)
    }

    pub fn take_off_equipment(&self, avatar_id: u32) -> Result<(), Retcode> {
        let equipment_uid = self.set_equipment_uid(Self::base_avatar_id(avatar_id), 0)?;

        let item_mgr = self.item_mgr.upgrade().unwrap();
        let item_mgr = item_mgr.borrow();
        let _ = item_mgr.assign_equipment(equipment_uid, 0);

        Ok(())
    }

    pub fn unlock_all_avatars(&self) {
        for id in EXCEL_COLLECTION
            .avatar_configs
            .iter()
            .filter(|c| c.avatar_id < 7000)
            .map(|c| c.avatar_id)
        {
            self.unlock_avatar(id).unwrap();
        }
    }

    pub fn unlock_avatar(&self, avatar_id: u32) -> anyhow::Result<()> {
        if self.has_avatar(avatar_id) {
            return Err(anyhow!("Avatar with id {avatar_id} is already unlocked"));
        }

        let mut player = self.player_info.borrow_mut();
        let avatar_comp = player.data.avatar_bin.as_mut().unwrap();

        if !EXCEL_COLLECTION
            .avatar_configs
            .iter()
            .any(|c| c.avatar_id == avatar_id)
        {
            return Err(anyhow!("Avatar with id {avatar_id} doesn't exist"));
        }

        avatar_comp.avatar_list.push(AvatarBin {
            avatar_type: AvatarType::AvatarFormalType.into(),
            avatar_id: Self::base_avatar_id(avatar_id),
            level: 80,
            exp: 0,
            promotion: 6,
            rank: 6,
            skill_tree_list: EXCEL_COLLECTION
                .avatar_skill_tree_configs
                .iter()
                .filter(|c| c.avatar_id == avatar_id && c.default_unlock)
                .map(|c| AvatarSkillTreeBin {
                    point_id: c.point_id,
                    level: c.level,
                })
                .collect(),
            equipment_unique_id: 0,
            relic_map: HashMap::new(),
        });

        Ok(())
    }

    pub fn avatar_list_proto(&self) -> Vec<Avatar> {
        let player_info = self.player_info.borrow();
        let avatar_comp = player_info.data.avatar_bin.as_ref().unwrap();

        avatar_comp
            .avatar_list
            .iter()
            .map(|a| Avatar {
                base_avatar_id: a.avatar_id,
                exp: a.exp,
                level: a.level,
                promotion: a.promotion,
                rank: a.rank,
                has_taken_reward_level_list: (0..6).collect(),
                equipment_unique_id: a.equipment_unique_id,
                skilltree_list: a
                    .skill_tree_list
                    .iter()
                    .map(|t| AvatarSkillTree {
                        point_id: t.point_id,
                        level: t.level,
                    })
                    .collect(),
                equip_relic_list: a
                    .relic_map
                    .iter()
                    .map(|(slot, uid)| EquipRelic {
                        r#type: *slot,
                        relic_unique_id: *uid,
                    })
                    .collect(),
                ..Default::default()
            })
            .collect()
    }

    pub fn avatar_list_battle_proto(&self, avatars: Vec<LineupAvatarBin>) -> Vec<BattleAvatar> {
        let player_info = self.player_info.borrow();
        let avatar_comp = player_info.data.avatar_bin.as_ref().unwrap();
        let item_comp = player_info.data.item_bin.as_ref().unwrap();
        let hero_comp = player_info.data.hero_bin.as_ref().unwrap();

        avatars
            .iter()
            .map(|l| {
                let avatar = avatar_comp
                    .avatar_list
                    .iter()
                    .find(|a| a.avatar_id == l.avatar_id)
                    .unwrap();

                let equipment = item_comp
                    .equipment_list
                    .iter()
                    .find(|e| e.unique_id == avatar.equipment_unique_id);

                BattleAvatar {
                    index: l.slot,
                    id: if l.avatar_id == 8001 {
                        hero_comp.cur_basic_type as u32
                    } else {
                        l.avatar_id
                    },
                    level: avatar.level,
                    promotion: avatar.promotion,
                    rank: avatar.rank,
                    hp: l.hp,
                    avatar_type: l.avatar_type,
                    sp: Some(AmountInfo {
                        cur_amount: l.sp,
                        max_amount: 10000,
                    }),
                    equipment_list: equipment.map_or(Vec::new(), |e| {
                        vec![BattleEquipment {
                            id: e.tid,
                            level: e.level,
                            promotion: e.promotion,
                            rank: e.rank,
                        }]
                    }),
                    skilltree_list: avatar
                        .skill_tree_list
                        .iter()
                        .map(|st| AvatarSkillTree {
                            point_id: st.point_id,
                            level: st.level,
                        })
                        .collect(),
                    relic_list: avatar
                        .relic_map
                        .iter()
                        .map(|(_, uid)| {
                            let relic = item_comp
                                .relic_list
                                .iter()
                                .find(|r| r.unique_id == *uid)
                                .unwrap();

                            BattleRelic {
                                id: relic.tid,
                                level: relic.level,
                                main_affix_id: relic.main_affix_id,
                                sub_affix_list: relic
                                    .sub_affix_list
                                    .iter()
                                    .map(|a| RelicAffix {
                                        affix_id: a.affix_id,
                                        step: a.step,
                                        cnt: a.cnt,
                                    })
                                    .collect(),
                                ..Default::default()
                            }
                        })
                        .collect(),
                    ..Default::default()
                }
            })
            .collect()
    }

    const fn base_avatar_id(avatar_id: u32) -> u32 {
        if avatar_id > 8000 {
            Self::HERO_AVATAR_ID
        } else {
            avatar_id
        }
    }

    fn create_comp(&self) {
        let mut player = self.player_info.borrow_mut();

        player.data.avatar_bin = Some(PlayerAvatarCompBin {
            avatar_list: Vec::new(),
        });
    }
}
