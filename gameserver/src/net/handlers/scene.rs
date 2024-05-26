use super::*;

pub async fn on_enter_scene_cs_req(session: &PlayerSession, body: &EnterSceneCsReq) -> Result<()> {println!("{:?}", body);
    let mut scene_mgr = session.context.scene_mgr.borrow_mut();
    match scene_mgr.enter_scene_packet(session, body.entry_id, Some(body.dehbihbbbgo)).await {
        Ok(_scene_info) => {
            session
                .send(CMD_ENTER_SCENE_SC_RSP, EnterSceneScRsp::default())
                .await
        }
        Err(retcode) => {
            session
                .send(
                    CMD_ENTER_SCENE_SC_RSP,
                    EnterSceneScRsp {
                        retcode: retcode as u32,
                        ..Default::default()
                    },
                )
                .await
        }
    }
}

pub async fn on_get_cur_scene_info_cs_req(
    session: &PlayerSession,
    _body: &GetCurSceneInfoCsReq,
) -> Result<()> {
    let scene_mgr = session.context.scene_mgr.borrow();

    session
        .send(
            CMD_GET_CUR_SCENE_INFO_SC_RSP,
            GetCurSceneInfoScRsp {
                retcode: 0,
                scene: Some(scene_mgr.cur_scene_info_proto()),
            },
        )
        .await
}

pub async fn on_scene_entity_move_cs_req(
    session: &PlayerSession,
    body: &SceneEntityMoveCsReq,
) -> Result<()> {
    let mut scene_mgr = session.context.scene_mgr.borrow_mut();
    let challenge_mgr = session.context.challenge_mgr.borrow();

    if !challenge_mgr.is_challenge() {
        body.entity_motion_list
            .iter()
            .filter(|e| e.motion.is_some())
            .for_each(|e| scene_mgr.entity_move(e, body.entry_id));
    }

    session
        .send(
            CMD_SCENE_ENTITY_MOVE_SC_RSP,
            SceneEntityMoveScRsp::default(),
        )
        .await
}

pub async fn on_enter_section_cs_req(
    session: &PlayerSession,
    body: &EnterSectionCsReq,
) -> Result<()> {
    let scene_mgr = session.context.scene_mgr.borrow();
    scene_mgr.enter_section(body.section_id);

    session
        .send(CMD_ENTER_SECTION_SC_RSP, EnterSectionScRsp::default())
        .await
}

pub async fn on_scene_cast_skill_cs_req(
    session: &PlayerSession,
    body: &SceneCastSkillCsReq,
) -> Result<()> {println!("{:?}", body);
    // let challenge_mgr = &mut session.context.challenge_mgr.borrow_mut();
    // if challenge_mgr.is_challenge() {
    //     return challenge_mgr.skill_cast(session, body).await;
    // }

    /*let scene_mgr = session.context.scene_mgr.borrow();
    

    let player_info = session.player_info();
    let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

    let avatar_manager = session.context.avatar_mgr.borrow();

    let lineup_bin = lineup_comp
        .lineup_list
        .iter()
        .find(|l| l.index == lineup_comp.cur_lineup_index)
        .unwrap();

    let mut battle = SceneBattleInfo {
        stage_id: 201012311,
        logic_random_seed: rand::thread_rng().next_u32() % 1000000,
        battle_id: 1,
        battle_avatar_list: avatar_manager.avatar_list_battle_proto(lineup_bin.avatar_list.clone()),
        ..Default::default()
    };

    if body.assist_monster_wave_list.len() == 0 {
        let mut battle_wave = SceneMonsterWave::default();
        for entity_id in &body.hit_target_entity_id_list {
            let Some((_, monster)) = scene_mgr.get_monster(entity_id.clone()) else {continue;};
            battle_wave.monster_list.push(SceneMonsterData {
                monster_id: monster.monster_id(),
                ..Default::default()
            });
        }
        if battle_wave.monster_list.len() > 0 {
            battle.monster_wave_list.push(battle_wave);
        }
    }

    for wave in &body.assist_monster_wave_list {
        let mut battle_wave = SceneMonsterWave::default();
        for entity_id in &wave.entity_id_list {
            let Some((_, monster)) = scene_mgr.get_monster(entity_id.clone()) else {continue;};
            battle_wave.monster_list.push(SceneMonsterData {
                monster_id: monster.monster_id(),
                ..Default::default()
            });
        }
        battle.monster_wave_list.push(battle_wave);
    }

    println!("{:#?}", battle);

    if battle.monster_wave_list.iter().map(|a| a.monster_list.clone()).flatten().collect::<Vec<_>>().len() == 0 {
        return session.send(
            CMD_SCENE_CAST_SKILL_SC_RSP, 
            SceneCastSkillScRsp {
                retcode: 0,
                attacked_group_id: body.attacked_group_id,
                ..Default::default()
            }
        ).await;
    }*/

    let scene_mgr = session.context.scene_mgr.borrow();
    let battle_mgr = session.context.battle_mgr.borrow();

    if body.skill_index > 0 { //todo: real manager for this tehnique
        let cur_lineup = session.context.lineup_mgr.borrow().cur_lineup_proto().clone();
        let mut player_info = session.player_info_mut();
        let lineup_bin = player_info.data.lineup_bin.as_mut().unwrap();
        let avatar_id = cur_lineup.avatar_list.iter().find(|a| a.slot == cur_lineup.leader_slot).unwrap().id;
        lineup_bin.buffs.push(
            LineupBuff {
                avatar_id,
                buff_id: avatar_id * 100 + body.skill_index,
                slot: cur_lineup.leader_slot
            }
        );
        println!("{:?}", lineup_bin.buffs);
    }

    session.send(
        CMD_SCENE_CAST_SKILL_SC_RSP, 
        SceneCastSkillScRsp {
            retcode: 0,
            attacked_group_id: body.attacked_group_id, //todo: ambush debuff
            battle_info: if body.assist_monster_wave_list.len() > 0 || body.hit_target_entity_id_list.iter().find(|a| scene_mgr.get_monster(**a).is_some()).is_some() {
                drop(scene_mgr);
                Some(battle_mgr.start_battle(session, body.assist_monster_wave_list.clone(), body.hit_target_entity_id_list.clone()))
            } else if scene_mgr.get_monster(body.caster_id).is_some() && body.hit_target_entity_id_list.len() > 0 {
                drop(scene_mgr);
                Some(battle_mgr.start_battle(session, body.assist_monster_wave_list.clone(), vec![body.caster_id]))
            } else {Option::None},
            ..Default::default()
        }
    ).await
}