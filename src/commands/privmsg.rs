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

        let mut targets = self.targets.values;
        if targets.len() == 1 {
            let recipient_string = targets.pop().unwrap();
            let recipient = MessageRecipient::from_string(recipient_string.clone());
            let message = Message {
                header: Some(sender.clone()),
                recipient,
                content: format!("PRIVMSG {} :{}", recipient_string, self.text),
            };

            outbox.send(message).await?;

            return Ok(CommandOutput::default());
        }

        for recipient in targets.into_iter() {
            let message = Message {
                header: Some(sender.clone()),
                recipient: MessageRecipient::from_string(recipient),
                content: self.text.clone(),
            };

            let _ = outbox.send(message).await;
        }

        Ok(CommandOutput::default())
    }
}
