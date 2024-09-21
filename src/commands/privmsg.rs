use std::{io::BufWriter, net::TcpStream, sync::mpsc::Sender};

use anyhow::anyhow;
use irc_parser::{types::CommaSeparatedList, FromIRCString};

use crate::internals::{ConnectionState, Message};

use super::RunCommand;

#[derive(FromIRCString)]
pub struct PrivMsgArgs {
    targets: CommaSeparatedList<String>,
    text: String,
}

impl RunCommand for PrivMsgArgs {
    fn run(
        self,
        state: &mut ConnectionState,
        _writer: &mut BufWriter<TcpStream>,
        messages_tx: &mut Sender<Message>,
    ) -> anyhow::Result<()> {
        let sender = state
            .nickname
            .as_ref()
            .ok_or(anyhow!("nickname must be known at this point"))?;

        let mut targets = self.targets.values;
        if targets.len() == 1 {
            let recipient = targets.pop().unwrap();
            let message = Message::private_message(Some(sender.clone()), recipient, self.text);
            messages_tx.send(message)?;

            return Ok(());
        }

        let send_failures = targets
            .into_iter()
            .map(|recipient| {
                let message =
                    Message::private_message(Some(sender.clone()), recipient, self.text.clone());
                messages_tx.send(message)
            })
            .filter(|res| res.is_err());

        send_failures
            .for_each(|failure| println!("failed to send message due to {}", failure.unwrap_err()));

        Ok(())
    }
}
