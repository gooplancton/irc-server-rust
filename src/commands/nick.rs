use std::{io::BufWriter, net::TcpStream, sync::mpsc::Sender};

use irc_parser::FromIRCString;

use crate::internals::{ConnectionState, Message};

use super::RunCommand;

#[derive(Debug, FromIRCString)]
pub struct NickArgs {
    nickname: String,
}

impl RunCommand for NickArgs {
    fn run(
        self,
        state: &mut ConnectionState,
        _writer: &mut BufWriter<TcpStream>,
        _messages_tx: &mut Sender<Message>,
    ) -> anyhow::Result<()> {
        state.nickname = Some(self.nickname);

        Ok(())
    }
}
