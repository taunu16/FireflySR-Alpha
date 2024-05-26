use common::data::{level::PropState, EXCEL_COLLECTION, LEVEL_TABLE};

use super::*;

pub async fn on_get_scene_map_info_cs_req(
    session: &PlayerSession,
    body: &GetSceneMapInfoCsReq,
) -> Result<()> {
    let mut infoes: Vec<SceneMapInfo> = vec![];

    for entry_id in &body.entry_id_list {
        let Some(entrance_config) = EXCEL_COLLECTION
            .map_entrance_configs
            .iter()
            .find(|c| c.id == *entry_id)
        else {
            infoes.push(SceneMapInfo {
                entry_id: *entry_id,
                ..Default::default()
            });
            continue;
        };

        let Some(floor) = LEVEL_TABLE.level_floors.get(&entrance_config.floor_id) else {
            infoes.push(SceneMapInfo {
                entry_id: *entry_id,
                ..Default::default()
            });
            continue;
        };

        let mut info = SceneMapInfo {
            entry_id: *entry_id,
            ..Default::default()
        };

        for i in 1..100 { //required for soulglad venue xD
            for j in 0..100 {
                info.lighten_section_list.push((i * 10000) + j)
            }
        }

        for i in 0..100 {
            info.lighten_section_list.push(i)
        }

        for group_fl in &floor.group_instance_list {
            let Some(group) = LEVEL_TABLE.get_level_group(entrance_config.floor_id, group_fl.id) else {continue;};
            
            info.maze_group_list.push(MazeGroup {
                group_id: group_fl.id,
                ..Default::default()
            });

            if let Some(prop_list) = &group.prop_list {
                info.maze_prop_list.append(&mut prop_list.iter().map(|prop| {
                    let state = session.player_info().data.scene_bin.as_ref()
                        .map(|a| a.scene_list.clone().into_iter().find(|s| s.floor_id == floor.floor_id))
                        .flatten()
                        .map(|a| {a.group_map.get(&group_fl.id).map(|a| a.clone())})
                        .flatten()
                        .map(|a| a.prop_list.iter().find(|p| p.entity_id == prop.id).map(|a| a.clone()))
                        .flatten()
                        .map(|a| a.state)
                        .unwrap_or(prop.state.clone() as u32);

                    MazePropState {
                        group_id: group_fl.id,
                        config_id: prop.id,
                        state: if prop.anchor_id.unwrap_or(0) > 0 { PropState::CheckPointEnable as u32 } else { state } 
                    }
                }).collect());
            }
        }

        infoes.push(info);
    }

    session
        .send(
            CMD_GET_SCENE_MAP_INFO_SC_RSP,
            GetSceneMapInfoScRsp {
                entry_id: body.entry_id,
                map_info_list: infoes,
                retcode: 0,
                ..Default::default()
            },
        )
        .await
}
