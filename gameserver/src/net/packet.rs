use anyhow::Result;
use paste::paste;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::Instrument;

use proto::*;

use super::handlers::*;
use super::PlayerSession;

const HEAD_MAGIC: u32 = 0x9D74C714;
const TAIL_MAGIC: u32 = 0xD7A152C8;

pub struct NetPacket {
    pub cmd_type: u16,
    pub head: Vec<u8>,
    pub body: Vec<u8>,
}

impl From<NetPacket> for Vec<u8> {
    fn from(value: NetPacket) -> Self {
        let mut out = Self::new();

        out.extend(HEAD_MAGIC.to_be_bytes());
        out.extend(value.cmd_type.to_be_bytes());
        out.extend((value.head.len() as u16).to_be_bytes());
        out.extend((value.body.len() as u32).to_be_bytes());
        out.extend(value.head);
        out.extend(value.body);
        out.extend(TAIL_MAGIC.to_be_bytes());
        out
    }
}

impl NetPacket {
    pub async fn read(stream: &mut TcpStream) -> std::io::Result<Self> {
        assert_eq!(stream.read_u32().await?, HEAD_MAGIC);
        let cmd_type = stream.read_u16().await?;

        let head_length = stream.read_u16().await? as usize;
        let body_length = stream.read_u32().await? as usize;

        let mut head = vec![0; head_length];
        stream.read_exact(&mut head).await?;

        let mut body = vec![0; body_length];
        stream.read_exact(&mut body).await?;

        assert_eq!(stream.read_u32().await?, TAIL_MAGIC);

        Ok(Self {
            cmd_type,
            head,
            body,
        })
    }
}

macro_rules! trait_handler {
    ($($name:ident $cmd_type:pat,)*) => {
        pub trait NetCommandHandler {
            $(
                paste! {
                    async fn [<on_$name:snake>](session: &PlayerSession, body: &$name) -> Result<()> {
                        [<on_$name:snake>](session, body).await
                    }
                }
            )*

            async fn on_message(session: &PlayerSession, cmd_type: u16, payload: Vec<u8>) -> Result<()> {
                use ::prost::Message;
                if PlayerSession::should_send_dummy_rsp(cmd_type) {
                    session.send_dummy_response(cmd_type).await?;
                    return Ok(());
                }
                match cmd_type {
                    // CMD_SCENE_ENTER_STAGE_CS_REQ => {
                    //     println!("{}: {}", cmd_type, rbase64::encode(&payload));
                    //     Ok(())
                    // }
                    CMD_LEAVE_CHALLENGE_CS_REQ => {
                        paste! {
                            on_leave_challenge_cs_req(session)
                                .instrument(tracing::info_span!(stringify!(on_leave_challenge_cs_req), cmd_type = cmd_type))
                                .await
                        }
                    }
                    CMD_PLAYER_HEART_BEAT_CS_REQ => {
                        on_player_heart_beat_cs_req(session, &PlayerHeartBeatCsReq::decode(&mut &payload[..])?).await
                    }
                    CMD_SCENE_ENTITY_MOVE_CS_REQ => {
                        on_scene_entity_move_cs_req(session, &SceneEntityMoveCsReq::decode(&mut &payload[..])?).await
                    }
                    CMD_P_V_E_BATTLE_RESULT_CS_REQ => {
                        let body = PveBattleResultCsReq::decode(&mut &payload[..])?;
                        paste! {
                            on_pve_battle_result_cs_req(session, &body)
                                .instrument(tracing::info_span!(stringify!(on_pve_battle_result_cs_req), cmd_type = cmd_type))
                                .await
                        }
                    }
                    $(
                        $cmd_type => {
                            let body = $name::decode(&mut &payload[..])?;
                            paste! {
                                Self::[<on_$name:snake>](session, &body)
                                    .instrument(tracing::info_span!(stringify!([<on_$name:snake>]), cmd_type = cmd_type))
                                    .await
                            }
                        }
                    )*
                    _ => {
                        tracing::warn!("Unknown command type: {cmd_type}");
                        Ok(())
                    },
                }
            }
        }
    };
}

macro_rules! trait_handler_auto {
    ($($name:ident;)*) => {
        paste! {
            trait_handler! { 
                $(
                    $name [<CMD_$name:snake:upper>],
                )*
            }
        }
    }
}

trait_handler_auto! {
    GetAvatarDataCsReq;
    TakeOffEquipmentCsReq;
    TakeOffRelicCsReq;
    DressAvatarCsReq;
    DressRelicAvatarCsReq;
    GetBagCsReq;
    GetAllLineupDataCsReq;
    JoinLineupCsReq;
    ChangeLineupLeaderCsReq;
    ReplaceLineupCsReq;
    QuitLineupCsReq;
    GetCurLineupDataCsReq;
    GetCurBattleInfoCsReq;
    GetMissionStatusCsReq;
    PlayerGetTokenCsReq;
    PlayerLoginCsReq;
    GetHeroBasicTypeInfoCsReq;
    GetBasicInfoCsReq;
    GetCurSceneInfoCsReq;
    StartCocoonStageCsReq;
    EnterSectionCsReq;
    GetSceneMapInfoCsReq;
    EnterSceneCsReq;
    GetTutorialGuideCsReq;
    UnlockTutorialGuideCsReq;
    GetTutorialCsReq;
    GetFriendListInfoCsReq;
    SendMsgCsReq;
    GetFirstTalkByPerformanceNpcCsReq;
    GetNpcTakenRewardCsReq;
    GetFirstTalkNpcCsReq;
    FinishTalkMissionCsReq;
    GetActivityScheduleConfigCsReq;
    StartChallengeCsReq;
    InteractPropCsReq;
    GroupStateChangeCsReq;
    GetShopListCsReq;
    SceneEnterStageCsReq;
    SceneCastSkillCsReq;
    GetCurChallengeCsReq;
    GetChallengeCsReq;
    GetArchiveDataCsReq;
    GetUpdatedArchiveDataCsReq;
}
