use common::data::EXCEL_COLLECTION;

use crate::game::managers::ChallengeManager;

use super::*;

pub async fn on_start_challenge_cs_req(
    session: &PlayerSession,
    body: &StartChallengeCsReq,
) -> Result<()> {println!("{:?}", body); //efocfephghg
    let mut challenge_mgr = session.context.challenge_mgr.borrow_mut();

    let lineup_mgr = session.context.lineup_mgr.borrow();
    let _ = lineup_mgr.replace_lineup(0, 0, &body.lineup_1.iter().enumerate().map(|(i, id)| LineupSlotInfo {id: *id, slot: i as u32, avatar_type: AvatarType::AvatarFormalType.into()}).collect(), ExtraLineupType::LineupChallenge);
    let _ = lineup_mgr.replace_lineup(0, 0, &body.lineup_1.iter().enumerate().map(|(i, id)| LineupSlotInfo {id: *id, slot: i as u32, avatar_type: AvatarType::AvatarFormalType.into()}).collect(), ExtraLineupType::LineupChallenge2);
    lineup_mgr.set_lineup_type(ExtraLineupType::LineupChallenge);

    let (challenge, lineup_list) = match challenge_mgr.start_challenge(body.challenge_id, body.lineup_1.clone(), body.lineup_2.clone()) {
        Ok(v) => v,
        Err(retcode) => return 
            session.send(
                CMD_START_CHALLENGE_SC_RSP, 
                StartChallengeScRsp {
                    retcode: retcode as u32,
                    ..Default::default()
                }
            ).await
    };

    let entrance_id = challenge.map_entrance_id;

    drop(challenge_mgr);

    let rsp = StartChallengeScRsp {
        retcode: 0,
        scene: Some(session.context.scene_mgr.borrow_mut().enter_scene_packet(session,  entrance_id, Option::None).await.unwrap()),
        lineup_list,
        // challenge_info: Some(ChallengeInfo {
        //     challenge_id: body.challenge_id,
        //     extra_lineup_type: ExtraLineupType::LineupNone as i32,
        //     status: ChallengeStatus::ChallengeDoing as i32,
        //     ..Default::default()
        // }),
        challenge_info: Some(ChallengeInfo {
            challenge_id: body.challenge_id,
            ogahmedokne: 1,
            oggklhikobo: 2100, // stage 1 score
            hlfjjlgkjci: 37, //stage 2 score
            kmjmkghelki: 1,
            status: ChallengeStatus::ChallengeDoing as i32,
            extra_lineup_type: ExtraLineupType::LineupNone as i32,
            ..Default::default()
        }),
        story_info: Some(Nlfkoddiafa {
            efocfephghg: Some(Pdeaocdjmag {
                lineup_1: body.lineup_1.clone(),
                lineup_2: body.lineup_2.clone(),
                ..Default::default()
            })
        })
    };println!("{:#?}", rsp);

    session.send(
        CMD_START_CHALLENGE_SC_RSP,
        rsp
    ).await
}

pub async fn on_get_challenge_cs_req(
    session: &PlayerSession,
    _body: &GetChallengeCsReq,
) -> Result<()> {
    session.send(
        CMD_GET_CHALLENGE_SC_RSP, 
        GetChallengeScRsp {
            retcode: 0,
            challenge_list: EXCEL_COLLECTION.challenge_maze_configs.iter().map(|c| Challenge {
                challenge_id: c.id,
                cpjbgfbhlpe: 1,
                hlfjjlgkjci: 2100, //score one
                igncekegblh: 2137, //
                ijjfhfincfd: 0, //stars 3-2 first 6-2 last 7-all
                oggklhikobo: 37, //score two
                oeeelifgfle: false,
                story_info: Option::None
            }).collect(),
            ..Default::default()
        }
    ).await
    // session.send_dummy(CMD_GET_CHALLENGE_SC_RSP).await
}

pub async fn on_leave_challenge_cs_req(
    session: &PlayerSession
) -> Result<()> {
    ChallengeManager::leave_challenge(session).await
}

pub async fn on_get_cur_challenge_cs_req(
    session: &PlayerSession,
    _body: &GetCurChallengeCsReq,
) -> Result<()> {
    let challenge_mgr = session.context.challenge_mgr.borrow();

    if !challenge_mgr.is_challenge() {
        return session.send(
            CMD_GET_CUR_CHALLENGE_SC_RSP, 
            GetCurChallengeScRsp {
                retcode: 0,
                ..Default::default()
            }
        ).await
    }

    let challenge = challenge_mgr.get_challenge();

    session.send(
        CMD_GET_CUR_CHALLENGE_SC_RSP, 
        GetCurChallengeScRsp {
            retcode: 0,
            challenge_info: Some(ChallengeInfo {
                challenge_id: challenge.id,
                ogahmedokne: 1,
                oggklhikobo: 2100,
                hlfjjlgkjci: 37,
                kmjmkghelki: 1,
                status: ChallengeStatus::ChallengeDoing as i32,
                extra_lineup_type: ExtraLineupType::LineupNone as i32,
                ..Default::default()
            }),
            lineup_list: vec![ChallengeManager::vec_to_lineup(challenge.lineup_1.clone(), ExtraLineupType::LineupChallenge), ChallengeManager::vec_to_lineup(challenge.lineup_2.clone(), ExtraLineupType::LineupChallenge2)]
        }
    ).await
}

pub async fn on_scene_enter_stage_cs_req(
    session: &PlayerSession,
    body: &SceneEnterStageCsReq,
) -> Result<()> {
    let battle_mgr = session.context.battle_mgr.borrow();
    let avatar_mgr = session.context.avatar_mgr.borrow();

    session.send(
        CMD_SCENE_ENTER_STAGE_SC_RSP,
        Nfcgjmfjdja {
            retcode: 0,
            //stage_id: body.stage_id,
            battle_info: Some(battle_mgr.start_stage_battle(avatar_mgr, body.stage_id))
        }
    ).await
}