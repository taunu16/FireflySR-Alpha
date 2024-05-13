use crate::{net::PlayerSession, util};
use anyhow::Result;
use proto::*;

use super::*;

pub struct CommandManager {
    item_mgr: Arc<AtomicRefCell<ItemManager>>,
}

impl CommandManager {
    pub fn new(item_mgr: Arc<AtomicRefCell<ItemManager>>) -> Self {
        Self { item_mgr }
    }

    pub async fn execute_command(&self, command: &str, session: &PlayerSession) -> Result<()> {
        let input = command.split(" ").collect::<Vec<&str>>();

        let (Some(category), Some(action)) = (input.get(0), input.get(1)) else {
            return self
                .send_text(session, "Usage: /[category] [action] [arg1] [arg2] ...")
                .await;
        };

        let args = &input[2..];
        if let Err(_) = match (*category, *action) {
            ("/relic", "give") => self.give_relic(args, session).await,
            _ => self.send_text(session, "Unknown command").await,
        } {
            return self
                .send_text(
                    session,
                    "Command execution failed. Re-check your input and try again.",
                )
                .await;
        }

        Ok(())
    }

    const GIVE_RELIC_USAGE: &'static str = "Usage: /relic give [id] [level] [main_affix] [sub_affix_count] [sub_affix1_id] [sub_affix1_cnt] ...";
    async fn give_relic(&self, args: &[&str], session: &PlayerSession) -> Result<()> {
        if args.len() < 4 {
            return self.send_text(session, Self::GIVE_RELIC_USAGE).await;
        }

        let id = args[0].parse::<u32>()?;
        let level = args[1].parse::<u32>()?;
        let main_affix = args[2].parse::<u32>()?;
        let sub_affix_count = args[3].parse::<usize>()?;

        if args.len() - 4 < (sub_affix_count * 2) as usize {
            return self.send_text(session, Self::GIVE_RELIC_USAGE).await;
        }

        let mut sub_affix_params = Vec::with_capacity(sub_affix_count);

        let args = &args[4..];
        for i in 0..sub_affix_count {
            let sub_affix_id = args[i * 2].parse::<u32>()?;
            let sub_affix_cnt = args[i * 2 + 1].parse::<u32>()?;

            sub_affix_params.push((sub_affix_id, sub_affix_cnt));
        }

        let item_mgr = self.item_mgr.borrow();
        item_mgr.give_relic(id, level, main_affix, sub_affix_params)?;

        session
            .send(
                CMD_PLAYER_SYNC_SC_NOTIFY,
                PlayerSyncScNotify {
                    relic_list: item_mgr.relic_list_proto(),
                    ..Default::default()
                },
            )
            .await?;

        self.send_text(session, "Relic added successfully").await
    }

    async fn send_text(&self, session: &PlayerSession, content: &str) -> Result<()> {
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
}
