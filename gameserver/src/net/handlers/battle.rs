use rand::RngCore;

use super::*;
use crate::game::{gameplay_conf, managers::ChallengeManager};

pub async fn on_start_cocoon_stage_cs_req(
    session: &PlayerSession,
    body: &StartCocoonStageCsReq,
) -> Result<()> {
    let player_info = session.player_info();
    let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

    let avatar_manager = session.context.avatar_mgr.borrow();

    let lineup_bin = lineup_comp
        .lineup_list
        .iter()
        .find(|l| l.index == lineup_comp.cur_lineup_index)
        .unwrap();

    // currently this, TODO: BattleManager
    let rsp = StartCocoonStageScRsp {
        retcode: 0,
        prop_entity_id: body.prop_entity_id,
        cocoon_id: body.cocoon_id,
        wave: body.wave,
        battle_info: Some(SceneBattleInfo {
            stage_id: 201012311,
            logic_random_seed: rand::thread_rng().next_u32() % 1000000,
            battle_id: 1,
            battle_avatar_list: avatar_manager.avatar_list_battle_proto(lineup_bin.avatar_list.clone()),
            monster_wave_list: gameplay_conf
                .monster_wave_list
                .iter()
                .map(|monster_list| SceneMonsterWave {
                    monster_list: monster_list
                        .iter()
                        .map(|id| SceneMonsterData {
                            monster_id: *id,
                            ..Default::default()
                        })
                        .collect(),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }),
    };

    session.send(CMD_START_COCOON_STAGE_SC_RSP, rsp).await
}

pub async fn on_get_cur_battle_info_cs_req(
    session: &PlayerSession,
    _body: &GetCurBattleInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_CUR_BATTLE_INFO_SC_RSP,
            GetCurBattleInfoScRsp {
                battle_info: Some(SceneBattleInfo::default()),
                ffbpkghgmjm: Some(Fjojkdhlonn::default()),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_pve_battle_result_cs_req(
    session: &PlayerSession,
    body: &PveBattleResultCsReq,
) -> Result<()> {
    let challenge_mgr = session.context.challenge_mgr.borrow();

    if challenge_mgr.is_challenge() {
        drop(challenge_mgr);
        return ChallengeManager::battle_result(session, body).await
    }

    session
        .send(
            CMD_P_V_E_BATTLE_RESULT_SC_RSP,
            PveBattleResultScRsp {
                retcode: 0,
                end_status: body.end_status,
                battle_id: body.battle_id,
                ..Default::default()
            },
        )
        .await
}
