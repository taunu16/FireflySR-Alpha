use super::*;

pub async fn on_get_first_talk_by_performance_npc_cs_req(
    session: &PlayerSession,
    body: &GetFirstTalkByPerformanceNpcCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_FIRST_TALK_BY_PERFORMANCE_NPC_SC_RSP,
            GetFirstTalkByPerformanceNpcScRsp {
                retcode: 0,
                npc_meet_status_list: body.kajphbfibik.iter().map(|id| 
                    Pghekcopokm {
                        ldcaeblnbco: *id,
                        is_meet: false
                    }
                ).collect()
            },
        )
        .await
}

//GetNpcTakenRewardCsReq
pub async fn on_get_npc_taken_reward_cs_req(
    session: &PlayerSession,
    body: &GetNpcTakenRewardCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_NPC_TAKEN_REWARD_SC_RSP,
            GetNpcTakenRewardScRsp {
                retcode: 0,
                npc_id: body.npc_id,
                talk_event_list: vec![]
            }
        ).await
}

pub async fn on_get_first_talk_npc_cs_req(
    session: &PlayerSession,
    body: &GetFirstTalkNpcCsReq
) -> Result<()> {
    session.send(
        CMD_GET_FIRST_TALK_NPC_SC_RSP,
        GetFirstTalkNpcScRsp {
            retcode: 0,
            npc_meet_status_list: body.series_id_list.iter().map(|q| NpcMeetStatus {
                series_id: *q,
                is_meet: false
            }).collect()
        }
    ).await
}

pub async fn on_finish_talk_mission_cs_req(
    session: &PlayerSession,
    body: &FinishTalkMissionCsReq
) -> Result<()> {
    session.send(
        CMD_FINISH_TALK_MISSION_SC_RSP, 
        FinishTalkMissionScRsp {
            cknnpogkead: body.cknnpogkead.clone(),
            retcode: 0,
            foaeacjbdcc: body.foaeacjbdcc.clone(),
            pjocnjdaigc: body.pjocnjdaigc
        }
    ).await
}