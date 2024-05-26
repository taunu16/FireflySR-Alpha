use std::collections::HashMap;

use proto::*;

use crate::net::PlayerSession;

use super::*;

pub struct LineupManager {
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
    extra_lineups: Arc<AtomicRefCell<HashMap<ExtraLineupType, LineupInfo>>>,
    lineup_type: Arc<AtomicRefCell<[ExtraLineupType]>>
}

impl LineupManager {
    const MP_LIMIT: u32 = 5;
    const AVATAR_SLOT_COUNT: u32 = 4;
    const DEFAULT_LINEUP_AVATARS: [u32; 4] = [1310, 0, 0, 0]; // firefly

    pub fn new(player_info: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self { player_info, extra_lineups: Arc::new(AtomicRefCell::new(HashMap::new())), lineup_type: Arc::new(AtomicRefCell::new([ExtraLineupType::LineupNone])) }
    }

    pub fn init_defaults(&self) {
        let mut player_info = self.player_info.borrow_mut();
        let avatar_comp = player_info.data.avatar_bin.as_ref().unwrap();

        player_info.data.lineup_bin = Some(PlayerLineupCompBin {
            lineup_list: vec![LineupBin {
                avatar_list: Self::DEFAULT_LINEUP_AVATARS
                    .iter()
                    .enumerate()
                    .filter(|(_, id)| **id != 0)
                    .map(|(i, id)| {
                        let avatar_bin = avatar_comp
                            .avatar_list
                            .iter()
                            .find(|a| a.avatar_id == *id)
                            .expect("Default lineup avatar is not present in PlayerAvatarCompBin");

                        LineupAvatarBin {
                            avatar_id: avatar_bin.avatar_id,
                            avatar_type: avatar_bin.avatar_type,
                            slot: i as u32,
                            hp: 10000,
                            sp: 10000,
                        }
                    })
                    .collect(),
                ..Default::default()
            }],
            cur_lineup_index: 0,
            mp: Self::MP_LIMIT,
            mp_max: Self::MP_LIMIT,
            buffs: vec![]
        });
    }

    pub fn set_cur_lineup_leader(&self, leader_slot: u32) -> Result<(), Retcode> {
        if !(0..Self::AVATAR_SLOT_COUNT).contains(&leader_slot) {
            return Err(Retcode::RetLineupNotValidLeader);
        }

        let mut player_info = self.player_info.borrow_mut();
        let lineup_comp = player_info.data.lineup_bin.as_mut().unwrap();

        let Some(lineup) = lineup_comp
            .lineup_list
            .iter_mut()
            .find(|l| l.index == lineup_comp.cur_lineup_index)
        else {
            return Err(Retcode::RetLineupIsEmpty);
        };

        lineup.leader_slot = leader_slot;
        Ok(())
    }

    pub fn join_lineup(&self, index: u32, slot: u32, base_avatar_id: u32) -> Result<(), Retcode> {
        if !(0..Self::AVATAR_SLOT_COUNT).contains(&slot) {
            return Err(Retcode::RetLineupInvalidMemberPos);
        }

        let mut player_info = self.player_info.borrow_mut();
        let lineup_comp = player_info.data.lineup_bin.as_mut().unwrap();
        let lineup_bin = lineup_comp
            .lineup_list
            .iter_mut()
            .find(|l| l.index == index)
            .ok_or(Retcode::RetLineupNotExist)?;

        if lineup_bin
            .avatar_list
            .iter()
            .any(|avatar| avatar.slot == slot)
        {
            return Err(Retcode::RetLineupAvatarAlreadyInit);
        }

        lineup_bin
            .avatar_list
            .push(Self::lineup_avatar_bin(base_avatar_id, slot));

        Ok(())
    }

    pub fn quit_lineup(&self, index: u32, base_avatar_id: u32) -> Result<(), Retcode> {
        let mut player_info = self.player_info.borrow_mut();
        let lineup_comp = player_info.data.lineup_bin.as_mut().unwrap();

        let lineup_bin = lineup_comp
            .lineup_list
            .iter_mut()
            .find(|l| l.index == index)
            .ok_or(Retcode::RetLineupNotExist)?;

        lineup_bin
            .avatar_list
            .retain(|a| a.avatar_id != base_avatar_id);

        Ok(())
    }

    pub fn set_lineup_type(&self, lineup_type: ExtraLineupType) {
        self.lineup_type.borrow_mut()[0] = lineup_type
    }

    pub fn replace_lineup(
        &self,
        index: u32,
        leader_slot: u32,
        replace_slot_list: &Vec<LineupSlotInfo>,
        lineup_type: ExtraLineupType
    ) -> Result<(), Retcode> {
        if !(0..Self::AVATAR_SLOT_COUNT).contains(&leader_slot) {
            return Err(Retcode::RetLineupInvalidMemberPos);
        }
        if lineup_type != ExtraLineupType::LineupNone {
            let mut lineups = self.extra_lineups.borrow_mut();
            if !lineups.contains_key(&lineup_type) {
                lineups.insert(lineup_type.clone(), LineupInfo {
                    index: 0,
                    extra_lineup_type: ExtraLineupType::LineupNone as i32, //lineup_type.clone() as i32,
                    leader_slot: 0,
                    mp: 5,
                    mp_max: 5,
                    ..Default::default()
                });
            }
            let lineup = lineups.get_mut(&lineup_type).unwrap();
            lineup.avatar_list.clear();
            for slot in replace_slot_list {
                lineup.avatar_list.push(LineupAvatar {
                    hp: 10000,
                    avatar_type: slot.avatar_type,
                    id: slot.id,
                    slot: slot.slot,
                    sp: Some(AmountInfo {
                        cur_amount: 10000 / 2,
                        max_amount: 10000
                    }),
                    ..Default::default()
                });
            }
            return Result::Ok(());
        }

        let mut player_info = self.player_info.borrow_mut();
        let lineup_comp = player_info.data.lineup_bin.as_mut().unwrap();
        let lineup_bin = lineup_comp
            .lineup_list
            .iter_mut()
            .find(|l| l.index == index)
            .ok_or(Retcode::RetLineupNotExist)?;

        lineup_bin.avatar_list.clear();
        for slot_info in replace_slot_list {
            if !(0..Self::AVATAR_SLOT_COUNT).contains(&slot_info.slot) {
                return Err(Retcode::RetLineupInvalidMemberPos);
            }

            lineup_bin
                .avatar_list
                .push(Self::lineup_avatar_bin(slot_info.id, slot_info.slot));
        }

        lineup_bin.leader_slot = leader_slot;
        Ok(())
    }

    pub async fn sync_cur_lineup(&self, session: &PlayerSession) -> anyhow::Result<()> {
        session
            .send(
                CMD_SYNC_LINEUP_NOTIFY,
                SyncLineupNotify {
                    lineup: Some(self.cur_lineup_proto()),
                    ..Default::default()
                },
            )
            .await
    }

    pub fn cur_lineup_proto(&self) -> LineupInfo {
        let lineup_type = self.lineup_type.borrow_mut()[0].clone();
        if lineup_type != ExtraLineupType::LineupNone {
            return self.extra_lineups.borrow().get(&lineup_type).unwrap_or(&LineupInfo::default()).clone()
        }

        let player_info = self.player_info.borrow();
        let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

        self.get_lineup_proto(lineup_comp.cur_lineup_index).unwrap()
    }

    pub fn get_lineup_proto(&self, index: u32) -> Option<LineupInfo> {
        let player_info = self.player_info.borrow();
        let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

        lineup_comp
            .lineup_list
            .iter()
            .find(|l| l.index == index)
            .map(|l| LineupInfo {
                avatar_list: l
                    .avatar_list
                    .iter()
                    .map(|a| LineupAvatar {
                        id: a.avatar_id,
                        avatar_type: a.avatar_type,
                        slot: a.slot,
                        hp: a.hp,
                        sp: Some(AmountInfo {
                            cur_amount: a.sp,
                            max_amount: 10000,
                        }),
                        ..Default::default()
                    })
                    .collect(),
                index: l.index,
                name: l.name.clone(),
                is_virtual: l.is_virtual,
                plane_id: l.plane_id,
                mp: lineup_comp.mp,
                mp_max: lineup_comp.mp_max,
                leader_slot: l.leader_slot,
                extra_lineup_type: l.extra_lineup_type.into(),
                ..Default::default()
            })
    }

    pub fn get_all_lineup_proto(&self) -> Vec<LineupInfo> {
        let player_info = self.player_info.borrow();
        let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

        lineup_comp
            .lineup_list
            .iter()
            .enumerate()
            .map(|(i, _)| self.get_lineup_proto(i as u32))
            .flatten()
            .collect()
    }

    #[must_use]
    const fn lineup_avatar_bin(id: u32, slot: u32) -> LineupAvatarBin {
        LineupAvatarBin {
            avatar_id: id,
            slot,
            hp: 10000,
            sp: 10000,
            avatar_type: 3,
        }
    }
}
