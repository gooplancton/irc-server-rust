use std::{
    io::{BufWriter, Write},
    net::TcpStream,
    sync::mpsc::Sender,
};

use irc_parser::FromIRCString;

use crate::{connection::state::ConnectionState, internals::server::Message};

use super::RunCommand;

#[derive(FromIRCString)]
pub struct PingArgs {
    token: String,
}

impl RunCommand for PingArgs {
    fn run(
        self,
        _state: &mut ConnectionState,
        writer: &mut BufWriter<TcpStream>,
        _messages_tx: &mut Sender<Message>,
    ) -> anyhow::Result<()> {
        let message = format!("server PONG {}\r\n", self.token);
        writer.write_all(message.as_bytes())?;

        Ok(())
    }
}
