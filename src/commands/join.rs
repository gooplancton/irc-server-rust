#[allow(dead_code)]

use std::io::Write;

use irc_parser::{types::SpaceSeparatedList, FromIRCString};

use super::RunCommand;

#[derive(Default, FromIRCString)]
pub struct JoinArgs {
    channels: SpaceSeparatedList<String>,
}

impl RunCommand for JoinArgs {
    fn run(
        self,
        _state: &mut crate::connection::state::ConnectionState,
        writer: &mut std::io::BufWriter<std::net::TcpStream>,
    ) -> anyhow::Result<()> {
        writer.write_all("JOIN\r\n".as_bytes())?;

        Ok(())
    }
}

