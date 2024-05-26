use common::data::EXCEL_COLLECTION;

use super::*;

pub async fn on_get_mission_status_cs_req(
    session: &PlayerSession,
    body: &GetMissionStatusCsReq,
) -> Result<()> {
    let rsp = GetMissionStatusScRsp {
        finished_main_mission_id_list: EXCEL_COLLECTION.main_mission_configs.iter().map(|m| m.main_mission_id).collect(),
        sub_mission_status_list: body
            .sub_mission_id_list
            .iter()
            .map(|id| Mission {
                id: *id,
                progress: 1,
                status: MissionStatus::MissionFinish.into(),
            })
            .collect(),
        mission_event_status_list: body
            .main_mission_id_list
            .iter()
            .map(|id| Mission {
                id: *id,
                progress: 1,
                status: MissionStatus::MissionFinish.into(),
            })
            .collect(),
        ..Default::default()
    };

    session.send(CMD_GET_MISSION_STATUS_SC_RSP, rsp).await
}