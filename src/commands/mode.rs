use std::{io::BufWriter, net::TcpStream, sync::mpsc::Sender};

use irc_parser::FromIRCString;

use crate::internals::{ConnectionState, Message};

use super::RunCommand;

#[derive(FromIRCString, Debug)]
pub struct ModeArgs {
    target: String,
    modestring: Option<String>,
}

impl RunCommand for ModeArgs {
    fn run(
        self,
        _state: &mut ConnectionState,
        _writer: &mut BufWriter<TcpStream>,
        _messages_tx: &mut Sender<Message>,
    ) -> anyhow::Result<()> {
        // dbg!(&self);

        Ok(())
    }
}
