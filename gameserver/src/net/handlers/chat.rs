use crate::{game::commands, util};

use super::*;

pub async fn on_send_msg_cs_req(session: &PlayerSession, body: &SendMsgCsReq) -> Result<()> {
    let _ = session.send(
        CMD_GET_PRIVATE_CHAT_HISTORY_SC_RSP,
        GetPrivateChatHistoryScRsp {
            contact_id: 13371337,
            chat_message_list: vec![ChatMessageData {
                sender_id: session.player_info().uid,
                message_type: body.message_type.clone(),
                timestamp: util::cur_timestamp_seconds() - 1,
                content: body.message_text.clone(),
                ..Default::default()
            }],
            ..Default::default()
        }
    ).await;

    if body.message_text.starts_with("/") {
        commands::execute_command(&body.message_text, session).await?;
    }

    session
        .send(
            CMD_SEND_MSG_SC_RSP,
            SendMsgScRsp {
                end_time: util::cur_timestamp_seconds(),
                retcode: 0,
            },
        )
        .await
}
