use std::{io::BufWriter, net::TcpStream, sync::mpsc::Sender};

use anyhow::anyhow;
use irc_parser::{types::CommaSeparatedList, FromIRCString};

use crate::{connection::state::ConnectionState, internals::server::Message};

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

        let messages = self.targets.values.into_iter().map(|recipient| Message {
            recipient,
            sender: sender.clone(),
            text: self.text.clone(),
        });

        messages.for_each(|message| {
            let _ = messages_tx.send(message);
        });

        Ok(())
    }
}
