use crate::internals::{ConnectionState, Message};
use bytes::Bytes;
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
        outbox: &Sender<Message>,
    ) -> anyhow::Result<CommandOutput> {
        let sender = state.nickname.clone().map(Bytes::from);
        if sender.is_none() {
            let message = Message::not_registered(state.user_id, None);
            let _ = outbox.send(message).await;
            return Ok(CommandOutput::default());
        }

        let nickname = state.nickname.as_ref().unwrap();
        outbox
            .send(Message::welcome(state.user_id, nickname))
            .await?;

        Ok(CommandOutput::default())
    }
}
