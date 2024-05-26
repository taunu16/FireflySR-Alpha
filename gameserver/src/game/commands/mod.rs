use anyhow::Result;
use proto::*;

use crate::{net::PlayerSession, util};

mod avatar;
mod relic;
mod reset;
mod scene;

macro_rules! commands {
    ($($category:ident $action:ident;)*) => {
        pub async fn execute_command(command: &str, session: &PlayerSession) -> Result<()> {
            let input = command[1..].split(" ").collect::<Vec<&str>>();

            let (Some(category), Some(action)) = (input.get(0), input.get(1)) else {
                let mut help_text = "Available Commands: ".to_string();
                $(
                    help_text.push_str(stringify!($category));
                    help_text.push_str(" ");
                    help_text.push_str(stringify!($action));
                    help_text.push_str("; ");
                )*
                let _ = send_text(session, &help_text).await;
                let _ = send_text(session, "Usage: /[category] [action] [arg1] [arg2] ...").await;
                return send_text(session, "Type /[category] [action] to get more detailed help.").await;
            };

            let args = &input[2..];
            if let Err(_) = match (*category, *action) {
                $(
                    (stringify!($category), stringify!($action)) => {
                        $category::$action(args, session).await
                    }
                )*,
                _ => send_text(session, "Unknown command").await,
            } {
                return send_text(
                    session,
                    "Command execution failed. Re-check your input and try again.",
                )
                .await;
            }

            Ok(())
        }
    };
}

commands! {
    avatar max_traces;
    relic give;
    scene enter;
    reset propstates;
}

async fn send_text(session: &PlayerSession, content: &str) -> Result<()> {
    session
        .send(
            CMD_GET_PRIVATE_CHAT_HISTORY_SC_RSP,
            GetPrivateChatHistoryScRsp {
                contact_id: 13371337,
                chat_message_list: vec![ChatMessageData {
                    sender_id: 13371337,
                    message_type: MsgType::CustomText.into(),
                    timestamp: util::cur_timestamp_seconds(),
                    content: content.to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await
}
