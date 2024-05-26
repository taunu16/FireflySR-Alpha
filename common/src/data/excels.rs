use std::collections::HashMap;

use serde::Deserialize;

use super::level::PropState;

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvatarConfig {
    #[serde(rename = "AIPath")]
    pub aipath: String,
    pub action_avatar_head_icon_path: String,
    #[serde(rename = "AdventurePlayerID")]
    pub adventure_player_id: u32,
    pub assist_bg_offset: Vec<f64>,
    pub assist_offset: Vec<f64>,
    pub avatar_base_type: String,
    pub avatar_cutin_bg_img_path: String,
    pub avatar_cutin_front_img_path: String,
    pub avatar_cutin_img_path: String,
    pub avatar_cutin_intro_text: TextMapHashEntry,
    pub avatar_desc: TextMapHashEntry,
    pub avatar_drop_offset: Vec<f64>,
    pub avatar_full_name: TextMapHashEntry,
    pub avatar_gacha_result_img_path: String,
    #[serde(rename = "AvatarID")]
    pub avatar_id: u32,
    pub avatar_initial_skin_desc: TextMapHashEntry,
    pub avatar_initial_skin_name: TextMapHashEntry,
    pub avatar_mini_icon_path: String,
    pub avatar_name: TextMapHashEntry,
    pub avatar_self_show_offset: Vec<f64>,
    pub avatar_side_icon_path: String,
    pub avatar_trial_offset: Vec<f64>,
    #[serde(rename = "AvatarVOTag")]
    pub avatar_votag: String,
    pub damage_type: String,
    pub damage_type_resistance: Vec<f64>,
    pub default_avatar_head_icon_path: String,
    pub default_avatar_model_path: String,
    pub exp_group: u32,
    pub json_path: String,
    pub manikin_json_path: String,
    pub max_promotion: u32,
    pub max_rank: u32,
    pub player_card_offset: Vec<f64>,
    #[serde(rename = "RankIDList")]
    pub rank_idlist: Vec<u32>,
    pub rarity: String,
    pub release: bool,
    pub reward_list: Vec<ItemEntry>,
    pub reward_list_max: Vec<ItemEntry>,
    #[serde(rename = "SPNeed")]
    pub sp_need: FixPoint,
    pub side_avatar_head_icon_path: String,
    pub skill_list: Vec<u32>,
    pub skilltree_prefab_path: String,
    #[serde(rename = "UIAvatarModelPath")]
    pub uiavatar_model_path: String,
    pub ultra_skill_cut_in_prefab_path: String,
    pub waiting_avatar_head_icon_path: String,
}

#[derive(Default, Deserialize)]
pub struct TextMapHashEntry {
    pub hash: i64,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct FixPoint {
    pub raw_value: f64,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ItemEntry {
    #[serde(rename = "ItemID")]
    item_id: u32,
    item_num: u32,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvatarSkillTreeConfig {
    pub anchor: String,
    pub pre_point: Vec<u32>,
    pub status_add_list: Vec<AvatarPropertyValue>,
    pub material_list: Vec<ItemEntry>,
    #[serde(rename = "LevelUpSkillID")]
    pub level_up_skill_id: Vec<u32>,
    pub icon_path: String,
    pub point_name: String,
    pub point_desc: String,
    pub ability_name: String,
    pub param_list: Vec<FixPoint>,
    #[serde(rename = "PointID")]
    pub point_id: u32,
    pub level: u32,
    #[serde(rename = "AvatarID")]
    pub avatar_id: u32,
    pub point_type: u32,
    pub max_level: u32,
    pub avatar_level_limit: u32,
    pub avatar_promotion_limit: u32,
    pub recommend_priority: u32,
    pub point_trigger_key: StringHash,
    pub default_unlock: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvatarPropertyValue {
    pub value: FixPoint,
    pub property_type: AvatarPropertyType,
}

#[derive(Default, Deserialize)]
pub enum AvatarPropertyType {
    #[default]
    Unknown = 0,
    MaxHP = 1,
    Attack = 2,
    Defence = 3,
    Speed = 4,
    CriticalChance = 5,
    CriticalDamage = 6,
    HealRatio = 7,
    StanceBreakAddedRatio = 8,
    SPRatio = 9,
    StatusProbability = 10,
    StatusResistance = 11,
    PhysicalAddedRatio = 12,
    PhysicalResistance = 13,
    FireAddedRatio = 14,
    FireResistance = 15,
    IceAddedRatio = 16,
    IceResistance = 17,
    ThunderAddedRatio = 18,
    ThunderResistance = 19,
    WindAddedRatio = 20,
    WindResistance = 21,
    QuantumAddedRatio = 22,
    QuantumResistance = 23,
    ImaginaryAddedRatio = 24,
    ImaginaryResistance = 25,
    BaseHP = 26,
    HPDelta = 27,
    BaseAttack = 28,
    AttackDelta = 29,
    BaseDefence = 30,
    DefenceDelta = 31,
    HPAddedRatio = 32,
    AttackAddedRatio = 33,
    DefenceAddedRatio = 34,
    BaseSpeed = 35,
    HealTakenRatio = 36,
    PhysicalResistanceDelta = 37,
    FireResistanceDelta = 38,
    IceResistanceDelta = 39,
    ThunderResistanceDelta = 40,
    WindResistanceDelta = 41,
    QuantumResistanceDelta = 42,
    ImaginaryResistanceDelta = 43,
    AllDamageReduce = 44,
    RelicValueExtraAdditionRatio = 45,
    EquipValueExtraAdditionRatio = 46,
    EquipExtraRank = 47,
    AvatarExtraRank = 48,
    AllDamageTypeAddedRatio = 49,
    SpeedAddedRatio = 50,
    SpeedDelta = 51,
    CriticalChanceBase = 52,
    CriticalDamageBase = 53,
    SPRatioBase = 54,
    HealRatioBase = 55,
    StatusProbabilityBase = 56,
    StatusResistanceBase = 57,
    BreakDamageAddedRatio = 58,
    BreakDamageAddedRatioBase = 59,
    MaxSP = 60,
    Count = 61,
    SpecialMaxSP = 62,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StringHash {
    pub hash: i64,
    pub is_empty: bool,
    pub hash_key: String,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct TutorialConfig {
    pub tutorial_json_path: String,
    pub trigger_params: Vec<TriggerParam>,
    pub finish_trigger_params: Vec<TriggerParam>,
    #[serde(rename = "TutorialID")]
    pub tutorial_id: u32,
    pub priority: u32,
    pub can_interrupt: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct TutorialGuideGroupConfig {
    #[serde(rename = "TutorialGuideIDList")]
    pub tutorial_guide_id_list: Vec<u32>,
    pub trigger_params: Vec<TriggerParam>,
    pub finish_trigger_params: Vec<TriggerParam>,
    #[serde(rename = "GroupID")]
    pub group_id: u32,
    pub tutorial_type: u32,
    #[serde(rename = "RewardID")]
    pub reward_id: u32,
    pub message_text: TextMapHashEntry,
    pub can_review: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct TriggerParam {
    pub trigger_param: String,
    pub trigger_type: String,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ItemConfig {
    pub item_icon_path: String,
    pub item_figure_icon_path: String,
    pub item_currency_icon_path: String,
    pub item_avatar_icon_path: String,
    pub custom_data_list: Vec<i64>,
    #[serde(rename = "ReturnItemIDList")]
    pub return_item_id_list: Vec<ReturnItemIdList>,
    #[serde(rename = "ID")]
    pub id: u32,
    pub item_main_type: XItemType,
    pub item_sub_type: XItemType,
    pub inventory_display_tag: u32,
    pub rarity: Rarity,
    pub purpose_type: u32,
    pub pile_limit: u32,
    //pub use_method: UseMethod,
    #[serde(rename = "UseDataID")]
    pub use_data_id: u32,
    pub item_group: u32,
    pub sell_type: SellType,
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    pub is_sellable: bool,
    pub is_show_red_dot: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReturnItemIdList {
    #[serde(rename = "ItemID")]
    pub item_id: u32,
    pub item_num: u32,
}

#[derive(Default, Deserialize, PartialEq)]
pub enum XItemType {
    #[serde(rename = "AetherSkill")]
    AetherSkill,
    #[serde(rename = "AetherSpirit")]
    AetherSpirit,
    #[serde(rename = "AvatarCard")]
    AvatarCard,
    Book,
    #[serde(rename = "ChatBubble")]
    ChatBubble,
    #[serde(rename = "ChessRogueDiceSurface")]
    ChessRogueDiceSurface,
    Display,
    Eidolon,
    Equipment,
    Food,
    #[serde(rename = "ForceOpitonalGift")]
    ForceOpitonalGift,
    Formula,
    Gift,
    #[serde(rename = "HeadIcon")]
    HeadIcon,
    #[default]
    Material,
    Mission,
    #[serde(rename = "MuseumExhibit")]
    MuseumExhibit,
    #[serde(rename = "MuseumStuff")]
    MuseumStuff,
    #[serde(rename = "MusicAlbum")]
    MusicAlbum,
    #[serde(rename = "PhoneTheme")]
    PhoneTheme,
    Relic,
    #[serde(rename = "RelicRarityShowOnly")]
    RelicRarityShowOnly,
    #[serde(rename = "RelicSetShowOnly")]
    RelicSetShowOnly,
    #[serde(rename = "RogueMedal")]
    RogueMedal,
    #[serde(rename = "TravelBrochurePaster")]
    TravelBrochurePaster,
    Usable,
    Virtual,
}

#[derive(Default, Deserialize)]
pub enum Rarity {
    #[default]
    Normal,
    #[serde(rename = "NotNormal")]
    NotNormal,
    Rare,
    #[serde(rename = "SuperRare")]
    SuperRare,
    #[serde(rename = "VeryRare")]
    VeryRare,
}

#[derive(Default, Deserialize)]
pub enum SellType {
    #[default]
    Destroy,
    Sell,
    #[serde(rename = "UnSellable")]
    UnSellable,
}


#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EquipmentConfig {
    pub avatar_base_type: String,
    pub avatar_detail_offset: Vec<f64>,
    pub battle_dialog_offset: Vec<f64>,
    pub coin_cost: u32,
    pub equipment_desc: TextMapHashEntry,
    #[serde(rename = "EquipmentID")]
    pub equipment_id: u32,
    pub equipment_name: TextMapHashEntry,
    pub exp_provide: u32,
    pub exp_type: u32,
    pub gacha_result_offset: Vec<f64>,
    pub image_path: String,
    pub item_right_panel_offset: Vec<f64>,
    pub max_promotion: u32,
    pub max_rank: u32,
    pub rank_up_cost_list: Vec<u32>,
    pub rarity: String,
    pub release: bool,
    #[serde(rename = "SkillID")]
    pub skill_id: u32,
    pub thumbnail_path: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RelicConfig {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "SetID")]
    pub set_id: u32,
    #[serde(rename = "Type")]
    pub relic_type: RelicType,
    pub rarity: String,
    pub main_affix_group: u32,
    pub sub_affix_group: u32,
    pub max_level: u32,
    pub exp_type: u32,
    pub exp_provide: i32,
    pub coin_cost: u32,
    pub mode: String,
}

#[derive(Deserialize)]
pub enum RelicType {
    HEAD,
    NECK,
    HAND,
    BODY,
    FOOT,
    OBJECT,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct MapEntranceConfig {
    #[serde(rename = "BeginMainMissionIDList")]
    pub begin_main_mission_idlist: Vec<u32>,
    pub entrance_type: String,
    #[serde(rename = "FinishMainMissionIDList")]
    pub finish_main_mission_idlist: Vec<u32>,
    #[serde(rename = "FinishSubMissionIDList")]
    pub finish_sub_mission_idlist: Vec<u32>,
    #[serde(rename = "FloorID")]
    pub floor_id: u32,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "PlaneID")]
    pub plane_id: u32,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct MainMissionConfig {
    pub sub_reward_list: Vec<u32>,
    #[serde(rename = "MainMissionID")]
    pub main_mission_id: u32,
    pub display_priority: u32,
    pub next_track_main_mission: u32,
    pub track_weight: u32,
    pub mission_advance: u32,
    #[serde(rename = "RewardID")]
    pub reward_id: u32,
    #[serde(rename = "DisplayRewardID")]
    pub display_reward_id: u32,
    pub mission_pack: u32,
    #[serde(rename = "ChapterID")]
    pub chapter_id: u32,
    pub is_in_raid: bool,
    pub is_display_activity_icon: bool,
    pub inited: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ActivityPanelConfig {
    #[serde(rename = "UIPrefab")]
    pub ui_prefab: String,
    pub type_param: Vec<u32>,
    pub unlock_conditions: String,
    pub tab_icon: String,
    #[serde(rename = "PanelID")]
    pub panel_id: u32,
    #[serde(rename = "Type")]
    pub main_mission_config_type: u32,
    #[serde(rename = "ActivityModuleID")]
    pub activity_module_id: u32,
    pub hide_quest: u32,
    pub sort_weight: u32,
    #[serde(rename = "ActivityThemeID")]
    pub activity_theme_id: u32,
    #[serde(rename = "ResidentPanelUnlockModuleID")]
    pub resident_panel_unlock_module_id: u32,
    #[serde(rename = "EarlyAccessContentID")]
    pub early_access_content_id: u32,
    pub display_item_manual_sort: bool,
    pub is_activity_have_resident_part: bool,
    pub is_resident_panel: bool,
    pub daily_hint: bool,
    pub is_skip_switch_story_line: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ChallengeMazeConfig {
    pub damage_type1: Vec<DamageType>,
    pub damage_type2: Vec<DamageType>,
    #[serde(rename = "ChallengeTargetID")]
    pub challenge_target_id: Vec<u32>,
    #[serde(rename = "MonsterID1")]
    pub monster_id1: Vec<Option<serde_json::Value>>,
    #[serde(rename = "MonsterID2")]
    pub monster_id2: Vec<Option<serde_json::Value>>,
    pub config_list1: Vec<u32>,
    #[serde(rename = "NpcMonsterIDList1")]
    pub npc_monster_id_list1: Vec<u32>,
    #[serde(rename = "EventIDList1")]
    pub event_id_list1: Vec<u32>,
    pub config_list2: Vec<u32>,
    #[serde(rename = "NpcMonsterIDList2")]
    pub npc_monster_id_list2: Vec<u32>,
    #[serde(rename = "EventIDList2")]
    pub event_id_list2: Vec<u32>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "GroupID")]
    pub group_id: u32,
    #[serde(rename = "MapEntranceID")]
    pub map_entrance_id: u32,
    #[serde(rename = "MapEntranceID2")]
    pub map_entrance_id2: u32,
    pub pre_level: u32,
    #[serde(rename = "PreChallengeMazeID")]
    pub pre_challenge_maze_id: u32,
    pub floor: u32,
    #[serde(rename = "RewardID")]
    pub reward_id: u32,
    pub stage_num: u32,
    pub challenge_count_down: u32,
    #[serde(rename = "MazeGroupID1")]
    pub maze_group_id1: u32,
    #[serde(rename = "MazeGroupID2")]
    pub maze_group_id2: u32,
    #[serde(rename = "MazeBuffID")]
    pub maze_buff_id: u32,
}

#[derive(Deserialize)]
pub enum DamageType {
    Fire,
    Ice,
    Imaginary,
    Physical,
    Quantum,
    Thunder,
    Wind,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct PlaneEventConfig {
    pub drop_list: Vec<u32>,
    pub display_item_list: Vec<Option<serde_json::Value>>,
    #[serde(rename = "EventID")]
    pub event_id: u32,
    pub world_level: u32,
    #[serde(rename = "StageID")]
    pub stage_id: u32,
    pub reward: u32,
    pub avatar_exp_reward: u32,
    pub is_use_monster_drop: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct InteractConfig {
    pub item_cost_list: Vec<ItemCostList>,
    #[serde(rename = "InteractID")]
    pub interact_id: u32,
    pub src_state: PropState,
    pub target_state: PropState,
    pub interact_cost_type: InteractCostType,
    pub is_event: bool,
}

#[derive(Default, Deserialize)]
pub enum InteractCostType {
    #[serde(rename = "CheckItem")]
    CheckItem,
    #[serde(rename = "CostItem")]
    CostItem,
    #[default]
    None,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ItemCostList {
    #[serde(rename = "ItemID")]
    pub item_id: u32,
    pub item_num: u32,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct RewardConfig {
    #[serde(rename = "RewardID")]
    pub reward_id: u32,
    pub hcoin: u32,
    #[serde(rename = "ItemID_1")]
    pub item_id_1: u32,
    #[serde(rename = "Count_1")]
    pub count_1: u32,
    #[serde(rename = "Level_1")]
    pub level_1: u32,
    #[serde(rename = "Rank_1")]
    pub rank_1: u32,
    #[serde(rename = "ItemID_2")]
    pub item_id_2: u32,
    #[serde(rename = "Count_2")]
    pub count_2: u32,
    #[serde(rename = "Level_2")]
    pub level_2: u32,
    #[serde(rename = "Rank_2")]
    pub rank_2: u32,
    #[serde(rename = "ItemID_3")]
    pub item_id_3: u32,
    #[serde(rename = "Count_3")]
    pub count_3: u32,
    #[serde(rename = "Level_3")]
    pub level_3: u32,
    #[serde(rename = "Rank_3")]
    pub rank_3: u32,
    #[serde(rename = "ItemID_4")]
    pub item_id_4: u32,
    #[serde(rename = "Count_4")]
    pub count_4: u32,
    #[serde(rename = "Level_4")]
    pub level_4: u32,
    #[serde(rename = "Rank_4")]
    pub rank_4: u32,
    #[serde(rename = "ItemID_5")]
    pub item_id_5: u32,
    #[serde(rename = "Count_5")]
    pub count_5: u32,
    #[serde(rename = "Level_5")]
    pub level_5: u32,
    #[serde(rename = "Rank_5")]
    pub rank_5: u32,
    #[serde(rename = "ItemID_6")]
    pub item_id_6: u32,
    #[serde(rename = "Count_6")]
    pub count_6: u32,
    #[serde(rename = "Level_6")]
    pub level_6: u32,
    #[serde(rename = "Rank_6")]
    pub rank_6: u32,
    pub is_special: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ShopConfig {
    pub shop_icon_path: String,
    pub shop_bar: String,
    pub limit_value1_list: Vec<u32>,
    pub limit_value2_list: Vec<Option<serde_json::Value>>,
    #[serde(rename = "ShopID")]
    pub shop_id: u32,
    pub shop_main_type: String,
    pub shop_type: u32,
    #[serde(rename = "ShopSortID")]
    pub shop_sort_id: u32,
    pub limit_type1: String,
    #[serde(rename = "ScheduleDataID")]
    pub schedule_data_id: u32,
    #[serde(rename = "ActivityModuleID")]
    pub activity_module_id: u32,
    pub is_open: bool,
    pub server_verification: bool,
    pub hide_remain_time: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ShopGoodsConfig {
    pub currency_list: Vec<u32>,
    pub currency_cost_list: Vec<u32>,
    pub limit_value1_list: Vec<u32>,
    pub limit_value2_list: Vec<Option<serde_json::Value>>,
    pub on_shelf_value1_list: Vec<u32>,
    #[serde(rename = "GoodsID")]
    pub goods_id: u32,
    #[serde(rename = "ItemID")]
    pub item_id: u32,
    #[serde(rename = "ItemGroupID")]
    pub item_group_id: u32,
    pub item_count: u32,
    pub level: u32,
    pub rank: u32,
    #[serde(rename = "GoodsSortID")]
    pub goods_sort_id: u32,
    pub limit_type1: String,
    pub limit_type2: String,
    pub on_shelf_type1: String,
    pub limit_times: u32,
    pub refresh_type: String,
    pub cycle_days: u32,
    pub refresh_offset: u32,
    #[serde(rename = "ShopID")]
    pub shop_id: u32,
    #[serde(rename = "ScheduleDataID")]
    pub schedule_data_id: u32,
    pub tag_type: u32,
    pub tag_param: u32,
    #[serde(rename = "ActivityModuleID")]
    pub activity_module_id: u32,
    pub is_limited_time_purchase: bool,
    pub is_on_sale: bool,
    pub is_new: bool,
}

#[derive(Default, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct MazePlaneConfig {
    #[serde(rename = "FloorIDList")]
    pub floor_id_list: Vec<u32>,
    #[serde(rename = "PlaneID")]
    pub plane_id: u32,
    pub plane_type: PlaneType,
    pub sub_type: u32,
    pub maze_pool_type: u32,
    #[serde(rename = "WorldID")]
    pub world_id: u32,
    #[serde(rename = "StartFloorID")]
    pub start_floor_id: u32,
}

#[derive(Default, Deserialize, Clone, Debug)]
pub enum PlaneType {
    Unknown = 0,
    #[serde(rename = "AetherDivide")]
    AetherDivide = 7,
    Challenge = 4,
    Maze = 2,
    Raid = 6,
    Rogue = 5,
    #[default]
    Town = 1,
    Train = 3,
    #[serde(rename = "TrialActivity")]
    TrialActivity = 8,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct StageConfig {
    pub level_graph_path: String,
    pub stage_ability_config: Vec<String>,
    pub sub_level_graphs: Vec<HashMap<String, Option<String>>>,
    pub monster_list: Vec<MonsterList>,
    pub level_lose_condition: Vec<String>,
    pub level_win_condition: Vec<String>,
    pub trial_avatar_list: Vec<u32>,
    #[serde(rename = "StageID")]
    pub stage_id: u32,
    pub stage_type: StageType,
    pub hard_level_group: u32,
    pub level: u32,
    pub elite_group: u32,
    pub battle_scoring_group: u32,
    pub monster_warning_ratio: f64,
    pub forbid_auto_battle: bool,
    pub release: bool,
    pub forbid_exit_battle: bool,
    pub reset_battle_speed: bool,
    pub processed_template_variables: ProcessedTemplateVariables,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct ProcessedTemplateVariables {
    #[serde(rename = "_Wave")]
    pub wave: u32,
    #[serde(rename = "_IsEliteBattle")]
    pub is_elite_battle: u32,
}

#[derive(Default, Deserialize, PartialEq)]
pub enum StageType {
    #[serde(rename = "AetherDivide")]
    AetherDivide,
    #[serde(rename = "BattleCollege")]
    BattleCollege,
    #[serde(rename = "BoxingClub")]
    BoxingClub,
    Challenge,
    #[serde(rename = "ClockParkActivity")]
    ClockParkActivity,
    Cocoon,
    #[serde(rename = "EvolveBuildActivity")]
    EvolveBuildActivity,
    #[serde(rename = "FantasticStory")]
    FantasticStory,
    #[serde(rename = "FarmElement")]
    FarmElement,
    #[serde(rename = "FeverTimeActivity")]
    FeverTimeActivity,
    #[serde(rename = "FightActivity")]
    FightActivity,
    Heliobus,
    #[default]
    Mainline,
    #[serde(rename = "PunkLord")]
    PunkLord,
    #[serde(rename = "RogueChallengeActivity")]
    RogueChallengeActivity,
    #[serde(rename = "RogueEndlessActivity")]
    RogueEndlessActivity,
    #[serde(rename = "RogueRelic")]
    RogueRelic,
    #[serde(rename = "StarFightActivity")]
    StarFightActivity,
    #[serde(rename = "StrongChallengeActivity")]
    StrongChallengeActivity,
    #[serde(rename = "TelevisionActivity")]
    TelevisionActivity,
    #[serde(rename = "TreasureDungeon")]
    TreasureDungeon,
    Trial,
    #[serde(rename = "VerseSimulation")]
    VerseSimulation,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct MonsterList {
    pub monster0: u32,
    pub monster1: u32,
    pub monster2: u32,
    pub monster3: u32,
    pub monster4: u32,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ChallengeStoryExtraConfig {
    #[serde(rename = "BattleTargetID")]
    pub battle_target_id: Vec<u32>,
    #[serde(rename = "ID")]
    pub id: u32,
    pub turn_limit: u32,
    pub clear_score: u32,
}