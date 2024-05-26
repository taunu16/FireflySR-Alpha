use proto::SceneGroupInfo;

use super::entity::{SceneActor, SceneEntity, SceneMonster, SceneNpc, SceneProp};

#[derive(Debug, Clone)]
pub struct SceneGroup {
    pub id: u32,
    pub actor_list: Vec<SceneActor>,
    pub prop_list: Vec<SceneProp>,
    pub npc_list: Vec<SceneNpc>,
    pub monster_list: Vec<SceneMonster>,
}

impl SceneGroup {
    pub fn info(&self) -> SceneGroupInfo {
        let mut entity_list = Vec::with_capacity(self.actor_list.len() + self.prop_list.len());
        entity_list.extend(self.actor_list.iter().map(|a| a.scene_entity_info_proto()));
        entity_list.extend(self.prop_list.iter().map(|p| p.scene_entity_info_proto()));
        entity_list.extend(self.npc_list.iter().map(|p| p.scene_entity_info_proto()));
        entity_list.extend(self.monster_list.iter().map(|p| p.scene_entity_info_proto()));

        SceneGroupInfo {
            state: 1,
            group_id: self.id,
            entity_list,
        }
    }
}
