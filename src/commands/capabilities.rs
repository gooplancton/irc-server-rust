use bytes::Bytes;
use irc_parser::FromIRCString;
use tokio::sync::mpsc::Sender;

use crate::internals::{message::MessageRecipient, ConnectionState, Message};

use super::{CommandOutput, RunCommand};

#[derive(FromIRCString)]
pub struct CapabilitiesArgs {
    subcommand: String,
}

static LS_RESPONSE: Bytes = Bytes::from_static(b"CAP * LS :");

impl RunCommand for CapabilitiesArgs {
    async fn run(
        self,
        state: &ConnectionState,
        outbox: Sender<Message>,
    ) -> anyhow::Result<CommandOutput> {
        match self.subcommand.as_str() {
            "LS" => {
                let message = Message {
                    header: None,
                    content: LS_RESPONSE.clone(),
                    recipient: MessageRecipient::UserId(state.user_id),
                };

                let _ = outbox.send(message).await;
            }
            "END" => {}
            _ => todo!(),
        };

        Ok(CommandOutput::default())
    }
}
