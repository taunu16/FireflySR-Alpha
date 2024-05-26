use super::*;

async fn on_get_rogue_handbook_data_cs_req(
    session: &PlayerSession,
    _body: &GetRogueHandbookDataCsReq,
) -> Result<()> {
    session.send(
        CMD_GET_ROGUE_HANDBOOK_DATA_SC_RSP,
        GetRogueHandbookDataScRsp {
            retcode: 0,
            handbook_info: Some(Ecboblbpopa {
                buff_list: vec![
                    Ifjpllmdhpl {
                        buff_id: 1000001
                    }
                ],
                ..Default::default()
            })
        }
    ).await
}