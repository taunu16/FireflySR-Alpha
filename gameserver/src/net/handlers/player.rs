use crate::util;

use super::*;

pub async fn on_get_basic_info_cs_req(
    session: &PlayerSession,
    _body: &GetBasicInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_BASIC_INFO_SC_RSP,
            GetBasicInfoScRsp {
                retcode: Retcode::RetSucc as u32,
                player_setting_info: Some(PlayerSettingInfo::default()),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_get_hero_basic_type_info_cs_req(
    session: &PlayerSession,
    _body: &GetHeroBasicTypeInfoCsReq,
) -> Result<()> {
    let player_info = session.player_info();
    let hero_basic_type_bin = player_info.data.hero_bin.as_ref().unwrap();

    session
        .send(
            CMD_GET_HERO_BASIC_TYPE_INFO_SC_RSP,
            GetHeroBasicTypeInfoScRsp {
                retcode: Retcode::RetSucc as u32,
                gender: hero_basic_type_bin.gender.into(),
                cur_basic_type: hero_basic_type_bin.cur_basic_type.into(),
                basic_type_info_list: hero_basic_type_bin
                    .basic_type_list
                    .iter()
                    .map(|b| HeroBasicTypeInfo {
                        basic_type: b.basic_type.into(),
                        rank: b.rank,
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_player_heart_beat_cs_req(
    session: &PlayerSession,
    body: &PlayerHeartBeatCsReq,
) -> Result<()> {
    session
        .context
        .on_player_heartbeat(body.client_time_ms)
        .await?;

    session
        .send(
            CMD_PLAYER_HEART_BEAT_SC_RSP,
            PlayerHeartBeatScRsp {
                retcode: 0,
                client_time_ms: body.client_time_ms,
                server_time_ms: util::cur_timestamp_ms(),
                download_data: Some(ClientDownloadData {
                    version: 51,
                    time: util::cur_timestamp_ms() as i64,
                    data: rbase64::decode("G0x1YVMBGZMNChoKBAQICHhWAAAAAAAAAAAAAAAod0AB/8IBAABDUy5Vbml0eUVuZ2luZS5HYW1lT2JqZWN0LkZpbmQoIlVJUm9vdC9BYm92ZURpYWxvZy9CZXRhSGludERpYWxvZyhDbG9uZSkiKTpHZXRDb21wb25lbnRJbkNoaWxkcmVuKHR5cGVvZihDUy5SUEcuQ2xpZW50LkxvY2FsaXplZFRleHQpKS50ZXh0ID0gIls8Y29sb3I9IzVlNjlmZj5yZXZlcnNlZHJvb21zPC9jb2xvcj5dIDxjb2xvcj0jQUNDQUM5PkY8L2NvbG9yPjxjb2xvcj0jOURDQkM5Pmk8L2NvbG9yPjxjb2xvcj0jOEZDREM5PnI8L2NvbG9yPjxjb2xvcj0jODFDRkM5PmU8L2NvbG9yPjxjb2xvcj0jNzJEMUM5PmY8L2NvbG9yPjxjb2xvcj0jNjREM0M5Pmw8L2NvbG9yPjxjb2xvcj0jNTZENUM5Pnk8L2NvbG9yPjxjb2xvcj0jMDBCRjczPlM8L2NvbG9yPjxjb2xvcj0jMjRDQjlFPlI8L2NvbG9yPi1BbHBoYSB8IGdpdGh1Yi5jb20vPGNvbG9yPSNmZmRmNTI+dGF1bnUxNjwvY29sb3I+IgAAAAAAAAAAAAEEEAAAACQAQAApQEAAKYBAACnAQABWAAEALIAAAR1AQQCkgEEA5ABAAOnAwQHpAMIB6UDCAawAAAEsgAAAH8BChRkAgAAMAAAABANDUwQMVW5pdHlFbmdpbmUEC0dhbWVPYmplY3QEBUZpbmQEKVVJUm9vdC9BYm92ZURpYWxvZy9CZXRhSGludERpYWxvZyhDbG9uZSkEF0dldENvbXBvbmVudEluQ2hpbGRyZW4EB3R5cGVvZgQEUlBHBAdDbGllbnQEDkxvY2FsaXplZFRleHQEBXRleHQU/zIBAABbPGNvbG9yPSM1ZTY5ZmY+cmV2ZXJzZWRyb29tczwvY29sb3I+XSA8Y29sb3I9I0FDQ0FDOT5GPC9jb2xvcj48Y29sb3I9IzlEQ0JDOT5pPC9jb2xvcj48Y29sb3I9IzhGQ0RDOT5yPC9jb2xvcj48Y29sb3I9IzgxQ0ZDOT5lPC9jb2xvcj48Y29sb3I9IzcyRDFDOT5mPC9jb2xvcj48Y29sb3I9IzY0RDNDOT5sPC9jb2xvcj48Y29sb3I9IzU2RDVDOT55PC9jb2xvcj48Y29sb3I9IzAwQkY3Mz5TPC9jb2xvcj48Y29sb3I9IzI0Q0I5RT5SPC9jb2xvcj4tQWxwaGEgfCBnaXRodWIuY29tLzxjb2xvcj0jZmZkZjUyPnRhdW51MTY8L2NvbG9yPgEAAAABAAAAAAAQAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAAAAAABAAAABV9FTlY=").unwrap()
                }),
            },
        )
        .await
}
