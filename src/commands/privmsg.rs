use anyhow::anyhow;
use irc_parser::{types::CommaSeparatedList, FromIRCString};
use tokio::sync::mpsc::Sender;

use crate::internals::{message::MessageRecipient, ConnectionState, Message};

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
        outbox: Sender<Message>,
    ) -> anyhow::Result<CommandOutput> {
        let sender = state
            .nickname
            .as_ref()
            .ok_or(anyhow!("nickname must be known at this point"))?;

        for recipient_string in self.targets.values.into_iter() {
            let recipient = MessageRecipient::from_string(recipient_string.clone());
            let message = Message {
                header: Some(sender.clone()),
                recipient,
                content: format!("PRIVMSG {} :{}", recipient_string, self.text),
            };

            let send_res = outbox.send(message).await;
            if let Err(err) = send_res {
                eprintln!("error sending message: {}", err);
            }
        }

        Ok(CommandOutput::default())
    }
}
