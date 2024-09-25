use bytes::Bytes;
use irc_parser::{types::SpaceSeparatedList, FromIRCString};
use tokio::sync::mpsc::Sender;

use crate::internals::{message::MessageRecipient, ConnectionState, Message};

use super::{CommandOutput, RunCommand};

#[derive(Default, FromIRCString)]
pub struct JoinArgs {
    channels: SpaceSeparatedList<String>,
}

static JOIN_RESPONSE: Bytes = Bytes::from_static(b"CAP * LS :");

impl RunCommand for JoinArgs {
    async fn run(
        self,
        state: &ConnectionState,
        outbox: &Sender<Message>,
    ) -> anyhow::Result<CommandOutput> {
        let join_message = Message {
            content: JOIN_RESPONSE.clone(),
            header: None,
            recipient: MessageRecipient::UserId(state.user_id),
        };

        outbox.send(join_message).await?;

        Ok(CommandOutput::join_channels(self.channels.values))
    }
}
