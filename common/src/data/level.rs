use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct RuntimeLevelFloorInfo {
    #[serde(rename = "FloorID")]
    pub floor_id: u32,
    pub start_group_index: u32,
    #[serde(rename = "StartAnchorID")]
    pub start_anchor_id: u32,
    pub group_instance_list: Vec<RuntimeGroupInstanceInfo>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct RuntimeGroupInstanceInfo {
    pub group_path: String,
    pub name: Option<String>,
    #[serde(rename = "ID")]
    pub id: u32,
    pub is_delete: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct RuntimeGroupInfo {
    pub load_side: GroupLoadSide,
    pub group_name: Option<String>,
    pub anchor_list: Option<Vec<LevelAnchorInfo>>,
    pub prop_list: Option<Vec<LevelPropInfo>>,
    #[serde(rename = "NPCList")]
    pub npc_list: Option<Vec<LevelNpcInfo>>,
    pub monster_list: Option<Vec<LevelMonsterInfo>>,
}

#[derive(Default, Deserialize, PartialEq, Eq)]
pub enum GroupLoadSide {
    #[default]
    Client,
    Server,
}


#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct LevelAnchorInfo {
    #[serde(rename = "ID")]
    pub id: u32,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub rot_x: f64,
    pub rot_y: f64,
    pub rot_z: f64,
    pub is_delete: bool,
}

#[derive(Default, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct LevelPropInfo {
    #[serde(rename = "ID")]
    pub id: u32,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub rot_x: f64,
    pub rot_y: f64,
    pub rot_z: f64,
    #[serde(rename = "PropID")]
    pub prop_id: u32,
    pub load_on_initial: bool,
    #[serde(rename = "CocoonID")]
    pub cocoon_id: u32,
    #[serde(rename = "MappingInfoID")]
    pub mapping_info_id: u32,
    #[serde(rename = "AnchorID")]
    pub anchor_id: Option<u32>,
    #[serde(rename = "EventID")]
    pub event_id: Option<u32>,
    #[serde(default)]
    pub state: PropState,
    pub init_level_graph: Option<String>
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
pub enum PropState {
    #[default]
    Closed = 0,
    Open = 1,
    Locked = 2,
    BridgeState1 = 3,
    BridgeState2 = 4,
    BridgeState3 = 5,
    BridgeState4 = 6,
    CheckPointDisable = 7,
    CheckPointEnable = 8,
    TriggerDisable = 9,
    TriggerEnable = 10,
    ChestLocked = 11,
    ChestClosed = 12,
    ChestUsed = 13,
    Elevator1 = 14,
    Elevator2 = 15,
    Elevator3 = 16,
    WaitActive = 17,
    EventClose = 18,
    EventOpen = 19,
    Hidden = 20,
    TeleportGate0 = 21,
    TeleportGate1 = 22,
    TeleportGate2 = 23,
    TeleportGate3 = 24,
    Destructed = 25,
    CustomState01 = 101,
    CustomState02 = 102,
    CustomState03 = 103,
    CustomState04 = 104,
    CustomState05 = 105,
    CustomState06 = 106,
    CustomState07 = 107,
    CustomState08 = 108,
    CustomState09 = 109,
}

#[derive(Default, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct LevelNpcInfo {
    #[serde(rename = "ID")]
    pub id: u32,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub rot_y: f64,
    #[serde(rename = "NPCID")]
    pub npc_id: u32,
    pub is_client_only: bool,
    pub is_delete: bool,
    // pub override_behavior_type: Option<String>
}

#[derive(Default, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct LevelMonsterInfo {
    #[serde(rename = "ID")]
    pub id: u32,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub rot_y: f64,
    #[serde(rename = "NPCMonsterID")]
    pub npc_monster_id: u32,
    #[serde(rename = "EventID")]
    pub event_id: u32,
    pub is_client_only: bool,
    pub is_delete: bool
}
