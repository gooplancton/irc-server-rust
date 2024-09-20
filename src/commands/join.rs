#[allow(dead_code)]
use std::io::Write;
use std::{io::BufWriter, net::TcpStream, sync::mpsc::Sender};

use irc_parser::{types::SpaceSeparatedList, FromIRCString};

use crate::{connection::state::ConnectionState, internals::server::Message};

use super::RunCommand;

#[derive(Default, FromIRCString)]
pub struct JoinArgs {
    channels: SpaceSeparatedList<String>,
}

impl RunCommand for JoinArgs {
    fn run(
        self,
        _state: &mut ConnectionState,
        writer: &mut BufWriter<TcpStream>,
        _messages_tx: &mut Sender<Message>,
    ) -> anyhow::Result<()> {
        writer.write_all("JOIN\r\n".as_bytes())?;

        Ok(())
    }
}
