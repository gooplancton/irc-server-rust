use bytes::Bytes;
use irc_parser::{types::CommaSeparatedList, FromIRCString};
use tokio::sync::mpsc::Sender;

use crate::internals::{ConnectionState, Message};

use super::{CommandOutput, RunCommand};

#[derive(FromIRCString)]
pub struct PrivMsgArgs {
    targets: CommaSeparatedList<String>,
    text: String,
}

impl RunCommand for PrivMsgArgs {
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

        let sender = sender.unwrap();
        for recipient_string in self.targets.values.into_iter() {
            let message = Message::privmsg(recipient_string, sender.clone(), &self.text);
            let _ = outbox.send(message).await;
        }

        Ok(CommandOutput::default())
    }
}
