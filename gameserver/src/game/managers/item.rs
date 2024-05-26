use anyhow::{anyhow, Result};
use common::data::{EXCEL_COLLECTION, XItemType};
use proto::*;
use crate::net::PlayerSession;

use super::*;

pub struct ItemManager {
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
}

impl ItemManager {
    pub fn new(player_info: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self { player_info }
    }

    pub fn init_defaults(&self) {
        self.create_comp();
        self.give_all_equipment();
        self.add_material(1, 1000000).unwrap();
    }

    pub async fn give_items(&self, session: &PlayerSession, items: Vec<Item>) -> Result<()> {
        for item in &items {
            if item.item_id == 0 {
                continue
            }

            let Some(config) = EXCEL_COLLECTION
                .item_configs
                .iter()
                .find(|c| c.id == item.item_id) else {
                return Err(anyhow!("Material with id {} doesn't exist", item.item_id));
            };

            match config.item_main_type {
                XItemType::Equipment => self.give_equipment_adv(item.item_id, item.level, item.rank, item.promotion),
                XItemType::Relic => self.give_relic(item.item_id, item.level, 1, vec![]),
                _ => self.add_material(item.item_id, item.num)
            }?
        }
        
        //TODO improve this so it does not send whole inventory
        session.send(
            CMD_PLAYER_SYNC_SC_NOTIFY, 
            PlayerSyncScNotify {
                equipment_list: self.equipment_list_proto(),
                relic_list: self.relic_list_proto(),
                material_list: self.material_list_proto(),
                ..Default::default()
            }
        ).await?;
    

        Ok(())
    }

    pub fn add_material(&self, material_id: u32, amount: u32) -> Result<()> {
        let mut player_info = self.player_info.borrow_mut();
        let item_comp = player_info.data.item_bin.as_mut().unwrap();

        if let Some(material) = item_comp
            .material_list
            .iter_mut()
            .find(|m| m.tid == material_id)
        {
            material.num += amount;
        } else {
            if !EXCEL_COLLECTION
                .item_configs
                .iter()
                .any(|c| c.id == material_id)
            {
                return Err(anyhow!("Material with id {material_id} doesn't exist"));
            }

            item_comp.material_list.push(MaterialBin {
                tid: material_id,
                num: amount,
            });
        }

        Ok(())
    }

    pub fn assign_equipment(&self, unique_id: u32, avatar_id: u32) -> Result<(), Retcode> {
        let mut player_info = self.player_info.borrow_mut();
        let item_comp = player_info.data.item_bin.as_mut().unwrap();

        let equipment = item_comp
            .equipment_list
            .iter_mut()
            .find(|e| e.unique_id == unique_id)
            .ok_or(Retcode::RetEquipmentNotExist)?;

        equipment.avatar_id = avatar_id;
        Ok(())
    }

    pub fn assign_relic(&self, unique_id: u32, avatar_id: u32) -> Result<(), Retcode> {
        let mut player_info = self.player_info.borrow_mut();
        let item_comp = player_info.data.item_bin.as_mut().unwrap();

        let relic = item_comp
            .relic_list
            .iter_mut()
            .find(|r| r.unique_id == unique_id)
            .ok_or(Retcode::RetRelicNotExist)?;

        relic.avatar_id = avatar_id;
        Ok(())
    }

    pub fn give_all_equipment(&self) {
        for config in EXCEL_COLLECTION
            .equipment_configs
            .iter()
            .filter(|c| c.release)
        {
            self.give_equipment(config.equipment_id).unwrap();
        }
    }

    pub fn give_relic(
        &self,
        id: u32,
        level: u32,
        affix_id: u32,
        sub_affix_list: Vec<(u32, u32)>,
    ) -> Result<()> {
        let unique_id = self.next_unique_id();

        let mut player_info = self.player_info.borrow_mut();
        let item_comp = player_info.data.item_bin.as_mut().unwrap();

        item_comp.relic_list.push(RelicBin {
            unique_id,
            tid: id,
            level,
            main_affix_id: affix_id,
            sub_affix_list: sub_affix_list
                .iter()
                .map(|(id, cnt)| RelicAffixBin {
                    affix_id: *id,
                    cnt: *cnt,
                    step: *cnt * 2,
                })
                .collect(),
            ..Default::default()
        });

        Ok(())
    }

    pub fn give_equipment(&self, equipment_id: u32) -> Result<()> {
        let config = EXCEL_COLLECTION
            .equipment_configs
            .iter()
            .find(|c| c.equipment_id == equipment_id)
            .ok_or(anyhow!("Equipment with id {equipment_id} doesn't exist"))?;

        let unique_id = self.next_unique_id();

        let mut player_info = self.player_info.borrow_mut();
        let item_comp = player_info.data.item_bin.as_mut().unwrap();

        item_comp.equipment_list.push(EquipmentBin {
            tid: config.equipment_id,
            unique_id,
            level: 80,
            rank: 1,
            promotion: 6,
            ..Default::default()
        });

        Ok(())
    }

    pub fn give_equipment_adv(&self, equipment_id: u32, level: u32, rank: u32, promotion: u32) -> Result<()> {
        let config = EXCEL_COLLECTION
            .equipment_configs
            .iter()
            .find(|c| c.equipment_id == equipment_id)
            .ok_or(anyhow!("Equipment with id {equipment_id} doesn't exist"))?;

        let unique_id = self.next_unique_id();

        let mut player_info = self.player_info.borrow_mut();
        let item_comp = player_info.data.item_bin.as_mut().unwrap();

        item_comp.equipment_list.push(EquipmentBin {
            tid: config.equipment_id,
            unique_id,
            level,
            rank,
            promotion,
            ..Default::default()
        });

        Ok(())
    }

    pub fn equipment_list_proto(&self) -> Vec<Equipment> {
        let player_info = self.player_info.borrow();
        let item_comp = player_info.data.item_bin.as_ref().unwrap();

        item_comp
            .equipment_list
            .iter()
            .map(|e| Equipment {
                tid: e.tid,
                unique_id: e.unique_id,
                level: e.level,
                exp: e.exp,
                promotion: e.promotion,
                rank: e.rank,
                is_protected: e.is_protected,
                equip_avatar_id: e.avatar_id,
                base_avatar_id: if e.avatar_id > 8000 {
                    8001
                } else {
                    e.avatar_id
                },
            })
            .collect()
    }

    pub fn material_list_proto(&self) -> Vec<Material> {
        let player_info = self.player_info.borrow();
        let item_comp = player_info.data.item_bin.as_ref().unwrap();

        item_comp
            .material_list
            .iter()
            .map(|m| Material {
                tid: m.tid,
                num: m.num,
                ..Default::default()
            })
            .collect()
    }

    pub fn relic_list_proto(&self) -> Vec<Relic> {
        let player_info = self.player_info.borrow();
        let item_comp = player_info.data.item_bin.as_ref().unwrap();

        item_comp
            .relic_list
            .iter()
            .map(|r| Relic {
                main_affix_id: r.main_affix_id,
                equip_avatar_id: r.avatar_id,
                base_avatar_id: if r.avatar_id > 8000 {
                    8001
                } else {
                    r.avatar_id
                },
                sub_affix_list: r
                    .sub_affix_list
                    .iter()
                    .map(|s| RelicAffix {
                        affix_id: s.affix_id,
                        cnt: s.cnt,
                        step: s.step,
                    })
                    .collect(),
                unique_id: r.unique_id,
                is_protected: r.is_protected,
                level: r.level,
                exp: r.exp,
                tid: r.tid,
                ..Default::default()
            })
            .collect()
    }

    fn create_comp(&self) {
        let mut player_info = self.player_info.borrow_mut();
        player_info.data.item_bin = Some(PlayerItemCompBin::default());
    }

    fn next_unique_id(&self) -> u32 {
        let mut player_info = self.player_info.borrow_mut();
        let item_comp = player_info.data.item_bin.as_mut().unwrap();

        item_comp.uid_counter += 1;
        item_comp.uid_counter
    }
}
