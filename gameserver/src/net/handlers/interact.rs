use common::data::{level::PropState, EXCEL_COLLECTION};
use paste::paste;
use super::*;

macro_rules! listify_rewards {
    ($reward:ident, $($num:pat),*) => {
        paste! {
            vec![
                $(
                    Item {
                        item_id: $reward.[<item_id_$num>],
                        num: $reward.[<count_$num>],
                        level: $reward.[<level_$num>],
                        rank: $reward.[<rank_$num>],
                        ..Default::default()
                    }
                ),*
            ].into_iter().filter(|a| a.item_id > 0).collect()
        }
    };
}

pub async fn on_interact_prop_cs_req(
    session: &PlayerSession,
    body: &InteractPropCsReq
) -> Result<()> {
    let player_info = session.player_info();
    let scene_mgr = session.context.scene_mgr.borrow();
    let mut state = PropState::Open;

    let (group_id, prop) = scene_mgr.get_prop(body.prop_entity_id).unwrap();

    if let Some(interact) = EXCEL_COLLECTION.interact_configs.iter().find(|i| i.interact_id == body.interact_id && i.src_state.clone() as u32 == prop.state.clone()) {
        state = interact.target_state.clone();
    }

    if let Some(event) = EXCEL_COLLECTION.plane_event_configs.iter().find(|e| e.event_id == prop.event_id() && e.world_level == player_info.data.basic_bin.as_ref().map(|a| a.world_level).unwrap_or(0)) {
        let reward = EXCEL_COLLECTION.reward_configs.iter().find(|r| r.reward_id == event.reward);
        let _ = session.send(
            CMD_SCENE_PLANE_EVENT_SC_NOTIFY, 
            ScenePlaneEventScNotify { 
                emeofonpphl: Some(ItemList {
                    item_list: event.drop_list.iter().map(|id|
                        Item {
                            item_id: *id,
                            num: 1,
                            ..Default::default()
                        }
                    ).collect()
                }),
                inpbbkjhegk: Some(ItemList {
                    item_list: if let Some(reward) = reward {
                        listify_rewards!(reward, 1, 2, 3, 4, 5, 6)
                    } else {vec![]}
                }),
                ..Default::default()
            }
        ).await;
    }

    drop(player_info);
    scene_mgr.set_prop_state(group_id, body.prop_entity_id, state.clone() as u32);

    if state == PropState::ChestUsed {
        let _ = session.send(
            CMD_SCENE_PLANE_EVENT_SC_NOTIFY, 
            ScenePlaneEventScNotify { 
                emeofonpphl: Some(ItemList {
                    item_list: vec![
                        Item {
                            item_id: 102,
                            num: 2137,
                            ..Default::default()
                        }
                    ]
                }),
                ..Default::default()
            }
        ).await;
    }

    session.send(
            CMD_SCENE_GROUP_REFRESH_SC_NOTIFY, 
            SceneGroupRefreshScNotify {
                refresh_group_list: vec![
                    SceneGroupRefreshInfo {
                        group_id,
                        state: state.clone() as u32,
                        refresh_entity_list: vec![
                            SceneGroupEntityRefreshInfo {
                                refresh_entity_info: Some(SceneEntityInfo {
                                    entity_id: body.prop_entity_id,
                                    group_id,
                                    prop: Some(ScenePropInfo {
                                        prop_id: prop.prop_id(),
                                        prop_state: state.clone() as u32,
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    }
                ]
            }
        ).await.unwrap();

    session
        .send(
            CMD_INTERACT_PROP_SC_RSP,
            InteractPropScRsp {
                retcode: 0,
                prop_entity_id: body.prop_entity_id,
                prop_state: state as u32
            }
        ).await
}

pub async fn on_group_state_change_cs_req(
    session: &PlayerSession,
    body: &GroupStateChangeCsReq
) -> Result<()> {println!("{:?}", body);
    let _ = session.send(
        CMD_GROUP_STATE_CHANGE_SC_NOTIFY,
        GroupStateChangeScNotify {
            group_state_info: body.group_state_info.clone()
        }
    ).await;

    session.send(
        CMD_GROUP_STATE_CHANGE_SC_RSP,
        GroupStateChangeScRsp {
            group_state_info: body.group_state_info.clone(),
            retcode: 0
        }
    ).await
}