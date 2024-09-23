use bytes::Bytes;
use irc_parser::FromIRCString;
use tokio::sync::mpsc::Sender;

use crate::internals::{message::MessageRecipient, ConnectionState, Message};

use super::{CommandOutput, RunCommand};

#[derive(FromIRCString)]
pub struct PingArgs {
    token: String,
}

impl RunCommand for PingArgs {
    async fn run(
        self,
        state: &ConnectionState,
        outbox: Sender<Message>,
    ) -> anyhow::Result<CommandOutput> {
        let content = Bytes::from(format!(":server PONG {}", self.token));
        let message = Message {
            header: None,
            content,
            recipient: MessageRecipient::UserId(state.user_id),
        };

        outbox.send(message).await?;

        Ok(CommandOutput::default())
    }
}
