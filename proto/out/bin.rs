#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerBasicCompBin {
    #[prost(uint32, tag = "1")]
    pub level: u32,
    #[prost(uint32, tag = "2")]
    pub exp: u32,
    #[prost(string, tag = "3")]
    pub nickname: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub login_times: u32,
    #[prost(int64, tag = "5")]
    pub created_timestamp: i64,
    #[prost(int64, tag = "6")]
    pub last_login_timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerAvatarCompBin {
    #[prost(message, repeated, tag = "1")]
    pub avatar_list: ::prost::alloc::vec::Vec<AvatarBin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvatarSkillTreeBin {
    #[prost(uint32, tag = "1")]
    pub point_id: u32,
    #[prost(uint32, tag = "2")]
    pub level: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvatarBin {
    #[prost(int32, tag = "1")]
    pub avatar_type: i32,
    #[prost(uint32, tag = "2")]
    pub avatar_id: u32,
    #[prost(uint32, tag = "3")]
    pub level: u32,
    #[prost(uint32, tag = "4")]
    pub exp: u32,
    #[prost(uint32, tag = "5")]
    pub promotion: u32,
    #[prost(uint32, tag = "6")]
    pub rank: u32,
    #[prost(message, repeated, tag = "7")]
    pub skill_tree_list: ::prost::alloc::vec::Vec<AvatarSkillTreeBin>,
    #[prost(uint32, tag = "8")]
    pub equipment_unique_id: u32,
    #[prost(map = "uint32, uint32", tag = "9")]
    pub relic_map: ::std::collections::HashMap<u32, u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerLineupCompBin {
    #[prost(message, repeated, tag = "1")]
    pub lineup_list: ::prost::alloc::vec::Vec<LineupBin>,
    #[prost(uint32, tag = "2")]
    pub cur_lineup_index: u32,
    #[prost(uint32, tag = "3")]
    pub mp: u32,
    #[prost(uint32, tag = "4")]
    pub mp_max: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineupBin {
    #[prost(message, repeated, tag = "1")]
    pub avatar_list: ::prost::alloc::vec::Vec<LineupAvatarBin>,
    #[prost(uint32, tag = "2")]
    pub index: u32,
    #[prost(uint32, tag = "3")]
    pub leader_slot: u32,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub is_virtual: bool,
    #[prost(uint32, tag = "6")]
    pub plane_id: u32,
    #[prost(int32, tag = "7")]
    pub extra_lineup_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineupAvatarBin {
    #[prost(uint32, tag = "1")]
    pub avatar_id: u32,
    #[prost(int32, tag = "2")]
    pub avatar_type: i32,
    #[prost(uint32, tag = "3")]
    pub slot: u32,
    #[prost(uint32, tag = "4")]
    pub hp: u32,
    #[prost(uint32, tag = "5")]
    pub sp: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroBasicTypeBin {
    #[prost(int32, tag = "1")]
    pub basic_type: i32,
    #[prost(uint32, tag = "2")]
    pub rank: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerHeroCompBin {
    #[prost(int32, tag = "1")]
    pub gender: i32,
    #[prost(int32, tag = "2")]
    pub cur_basic_type: i32,
    #[prost(message, repeated, tag = "3")]
    pub basic_type_list: ::prost::alloc::vec::Vec<HeroBasicTypeBin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipmentBin {
    #[prost(uint32, tag = "1")]
    pub unique_id: u32,
    #[prost(uint32, tag = "2")]
    pub tid: u32,
    #[prost(uint32, tag = "3")]
    pub level: u32,
    #[prost(uint32, tag = "4")]
    pub exp: u32,
    #[prost(uint32, tag = "5")]
    pub promotion: u32,
    #[prost(uint32, tag = "6")]
    pub rank: u32,
    #[prost(uint32, tag = "7")]
    pub avatar_id: u32,
    #[prost(bool, tag = "8")]
    pub is_protected: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaterialBin {
    #[prost(uint32, tag = "1")]
    pub tid: u32,
    #[prost(uint32, tag = "2")]
    pub num: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelicAffixBin {
    #[prost(uint32, tag = "1")]
    pub affix_id: u32,
    #[prost(uint32, tag = "2")]
    pub cnt: u32,
    #[prost(uint32, tag = "3")]
    pub step: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelicBin {
    #[prost(uint32, tag = "1")]
    pub unique_id: u32,
    #[prost(uint32, tag = "2")]
    pub tid: u32,
    #[prost(uint32, tag = "3")]
    pub level: u32,
    #[prost(uint32, tag = "4")]
    pub exp: u32,
    #[prost(uint32, tag = "5")]
    pub avatar_id: u32,
    #[prost(uint32, tag = "6")]
    pub main_affix_id: u32,
    #[prost(message, repeated, tag = "7")]
    pub sub_affix_list: ::prost::alloc::vec::Vec<RelicAffixBin>,
    #[prost(bool, tag = "8")]
    pub is_protected: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerItemCompBin {
    #[prost(uint32, tag = "1")]
    pub uid_counter: u32,
    #[prost(message, repeated, tag = "2")]
    pub equipment_list: ::prost::alloc::vec::Vec<EquipmentBin>,
    #[prost(message, repeated, tag = "3")]
    pub material_list: ::prost::alloc::vec::Vec<MaterialBin>,
    #[prost(message, repeated, tag = "4")]
    pub relic_list: ::prost::alloc::vec::Vec<RelicBin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TutorialBin {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(int32, tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TutorialGuideBin {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
    #[prost(int32, tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerTutorialCompBin {
    #[prost(message, repeated, tag = "1")]
    pub tutorial_list: ::prost::alloc::vec::Vec<TutorialBin>,
    #[prost(message, repeated, tag = "2")]
    pub tutorial_guide_list: ::prost::alloc::vec::Vec<TutorialGuideBin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VectorBin {
    #[prost(sint32, tag = "1")]
    pub x: i32,
    #[prost(sint32, tag = "2")]
    pub y: i32,
    #[prost(sint32, tag = "3")]
    pub z: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MotionBin {
    #[prost(message, optional, tag = "1")]
    pub pos: ::core::option::Option<VectorBin>,
    #[prost(message, optional, tag = "2")]
    pub rot: ::core::option::Option<VectorBin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SceneBin {
    #[prost(uint32, tag = "1")]
    pub plane_id: u32,
    #[prost(uint32, tag = "2")]
    pub floor_id: u32,
    #[prost(uint32, repeated, tag = "3")]
    pub lighten_section_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "4")]
    pub unlocked_teleport_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(int64, tag = "5")]
    pub last_enter_time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerSceneCompBin {
    #[prost(uint32, tag = "1")]
    pub cur_entry_id: u32,
    #[prost(message, optional, tag = "2")]
    pub cur_position: ::core::option::Option<MotionBin>,
    #[prost(message, repeated, tag = "3")]
    pub scene_list: ::prost::alloc::vec::Vec<SceneBin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDataBin {
    #[prost(message, optional, tag = "1")]
    pub basic_bin: ::core::option::Option<PlayerBasicCompBin>,
    #[prost(message, optional, tag = "2")]
    pub avatar_bin: ::core::option::Option<PlayerAvatarCompBin>,
    #[prost(message, optional, tag = "3")]
    pub lineup_bin: ::core::option::Option<PlayerLineupCompBin>,
    #[prost(message, optional, tag = "4")]
    pub hero_bin: ::core::option::Option<PlayerHeroCompBin>,
    #[prost(message, optional, tag = "5")]
    pub item_bin: ::core::option::Option<PlayerItemCompBin>,
    #[prost(message, optional, tag = "6")]
    pub tutorial_bin: ::core::option::Option<PlayerTutorialCompBin>,
    #[prost(message, optional, tag = "7")]
    pub scene_bin: ::core::option::Option<PlayerSceneCompBin>,
}
