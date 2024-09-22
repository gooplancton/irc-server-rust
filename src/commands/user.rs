use crate::{
    consts::RPL_WELCOME,
    internals::{message::MessageRecipient, ConnectionState, Message},
};
use anyhow::bail;
use irc_parser::FromIRCString;
use tokio::sync::mpsc::Sender;

use super::{CommandOutput, RunCommand};

#[derive(Debug, FromIRCString)]
pub struct UserArgs {
    _username: String,
    _unused1: String,
    _unused2: String,
    _realname: String,
}

impl RunCommand for UserArgs {
    async fn run(
        self,
        state: &ConnectionState,
        outbox: Sender<Message>,
    ) -> anyhow::Result<CommandOutput> {
        if state.nickname.is_none() {
            bail!("user has not yet provided a unique nickname");
        }

        let nickname = state.nickname.as_ref().unwrap();
        let content = format!("{} {} :Welcome {}\r\n", RPL_WELCOME, nickname, nickname);
        let message = Message {
            content,
            header: None,
            recipient: MessageRecipient::UserId(state.user_id),
        };

        outbox.send(message).await?;

        Ok(CommandOutput::default())
    }
}
