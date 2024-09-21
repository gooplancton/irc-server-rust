use std::{
    io::{BufWriter, Write},
    net::TcpStream,
    sync::mpsc::Sender,
};

use irc_parser::FromIRCString;

use crate::internals::{ConnectionState, Message};

use super::RunCommand;

#[derive(FromIRCString)]
pub struct CapabilitiesArgs {
    subcommand: String,
}

impl RunCommand for CapabilitiesArgs {
    fn run(
        self,
        _state: &mut ConnectionState,
        writer: &mut BufWriter<TcpStream>,
        _messages_tx: &mut Sender<Message>,
    ) -> anyhow::Result<()> {
        match self.subcommand.as_str() {
            "LS" => writer.write_all("CAP * LS :\r\n".as_bytes())?,
            "END" => {}
            _ => todo!(),
        };

        Ok(())
    }
}
