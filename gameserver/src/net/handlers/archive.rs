use super::*;

fn get_archive_data() -> ArchiveData {
    ArchiveData {
        lcfncpmjick: vec![40349, 59504, 443],
        ..Default::default()
    }
}

pub async fn on_get_archive_data_cs_req(
    session: &PlayerSession,
    _body: &GetArchiveDataCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_ARCHIVE_DATA_SC_RSP,
            GetArchiveDataScRsp {
                retcode: 0,
                archive_data: Some(get_archive_data())
            }
        ).await
}

pub async fn on_get_updated_archive_data_cs_req(
    session: &PlayerSession,
    _body: &GetUpdatedArchiveDataCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_UPDATED_ARCHIVE_DATA_SC_RSP,
            GetUpdatedArchiveDataScRsp {
                retcode: 0,
                archive_data: Some(get_archive_data())
            }
        ).await
}

// pub async fn on_get_jukebox_data_cs_req(
//     session: &PlayerSession,
//     _body: &GetJukeboxDataCsReq,
// ) -> Result<()> {
//     session.send(
//         CMD_GET_JUKEBOX_DATA_SC_RSP,
//         GetJukeboxDataScRsp {
//             retcode: 0,
//             mecnaihkejf: 210000,
//             music_list: EXCEL.background_music_config.iter().map(|v| 
//                 UnlockedMusic {
//                     id: v.id,
//                     group_id: v.group_id,
//                     gnkbckejghb: true 
//                 }
//             ).collect()
//         } 
//     ).await
// }

// pub async fn on_unlock_back_ground_music_cs_req(
//     session: &PlayerSession,
//     _body: &UnlockBackGroundMusicCsReq,
// ) -> Result<()> {
//     session.send(
//         CMD_UNLOCK_BACK_GROUND_MUSIC_SC_RSP,
//         UnlockBackGroundMusicScRsp {
//             retcode: 0,
//             unlocked_ids: EXCEL.background_music_config.iter().map(|v| v.id).collect(),
//             music_list: EXCEL.background_music_config.iter().map(|v| 
//                 UnlockedMusic {
//                     id: v.id,
//                     group_id: v.group_id,
//                     gnkbckejghb: true 
//                 }
//             ).collect()
//         } 
//     ).await
// }