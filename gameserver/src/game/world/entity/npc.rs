use proto::{MotionInfo, SceneEntityInfo, SceneNpcInfo, Vector, VectorBin};

use super::{EntityType, SceneEntity};

#[derive(Debug, Clone)]
pub struct SceneNpc {
    position: VectorBin,
    rotation: VectorBin,
    entity_id: u32,
    inst_id: u32,
    group_id: u32,
    npc_id: u32
}

impl SceneNpc {
    pub fn new(entity_id: u32, inst_id: u32, group_id: u32, npc_id: u32) -> Self {
        Self {
            position: VectorBin::default(),
            rotation: VectorBin::default(),
            entity_id,
            inst_id,
            group_id,
            npc_id
        }
    }
}

impl SceneEntity for SceneNpc {
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
            npc: Some(SceneNpcInfo {
                npc_id: self.npc_id,
                extra_info: Option::None
            }),
            ..Default::default()
        }
    }

    fn entity_type(&self) -> EntityType {
        EntityType::Actor
    }
}
