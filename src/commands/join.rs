use irc_parser::{types::SpaceSeparatedList, FromIRCString};
use tokio::sync::mpsc::Sender;

use crate::internals::{message::MessageRecipient, ConnectionState, Message};

use super::{CommandOutput, RunCommand};

#[derive(Default, FromIRCString)]
pub struct JoinArgs {
    channels: SpaceSeparatedList<String>,
}

impl RunCommand for JoinArgs {
    async fn run(
        self,
        state: &ConnectionState,
        outbox: Sender<Message>,
    ) -> anyhow::Result<CommandOutput> {
        let join_message = Message {
            content: "JOIN\r\n".to_owned(),
            header: None,
            recipient: MessageRecipient::UserId(state.user_id),
        };

        outbox.send(join_message).await?;

        Ok(CommandOutput::join_channels(self.channels.values))
    }
}
