use proto::{MotionInfo, SceneEntityInfo, SceneNpcMonsterInfo, Vector, VectorBin};

use super::{EntityType, SceneEntity};

#[derive(Debug, Clone)]
pub struct SceneMonster {
    position: VectorBin,
    rotation: VectorBin,
    entity_id: u32,
    inst_id: u32,
    group_id: u32,
    pub monster_id: u32,
    pub event_id: u32,
    pub stage_id: u32,
    world_level: u32,
}

impl SceneMonster {
    pub fn new(entity_id: u32, inst_id: u32, group_id: u32, monster_id: u32, event_id: u32, world_level: u32) -> Self {
        Self {
            position: VectorBin::default(),
            rotation: VectorBin::default(),
            entity_id,
            inst_id,
            group_id,
            monster_id,
            event_id,
            stage_id: event_id * 10 + world_level,
            world_level
        }
    }

    pub fn monster_id(&self) -> u32 {
        self.monster_id
    }

    pub fn inst_id(&self) -> u32 {
        self.inst_id
    }
}

impl SceneEntity for SceneMonster {
    fn position(&self) -> VectorBin {
        self.position.clone()
    }

    fn rotation(&self) -> VectorBin {
        self.rotation.clone()
    }

    fn set_position(&mut self, x: i32, y: i32, z: i32) {
        self.position = VectorBin { x, y, z }
    }

    fn set_rotation(&mut self, x: i32, y: i32, z: i32) {
        self.rotation = VectorBin { x, y, z }
    }

    fn entity_id(&self) -> u32 {
        self.entity_id
    }

    fn scene_entity_info_proto(&self) -> SceneEntityInfo {
        SceneEntityInfo {
            motion: Some(MotionInfo {
                pos: Some(Vector {
                    x: self.position.x,
                    y: self.position.y,
                    z: self.position.z,
                }),
                rot: Some(Vector {
                    x: self.rotation.x,
                    y: self.rotation.y,
                    z: self.rotation.z,
                }),
            }),
            entity_id: self.entity_id,
            inst_id: self.inst_id,
            group_id: self.group_id,
            npc_monster: Some(SceneNpcMonsterInfo {
                event_id: self.event_id,
                monster_id: self.monster_id,
                world_level: self.world_level,
                extra_info: Option::None,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn entity_type(&self) -> EntityType {
        EntityType::Prop
    }
}