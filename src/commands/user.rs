#![allow(dead_code)]

use irc_parser::FromIRCString;

use super::RunCommand;

#[derive(Debug, FromIRCString)]
pub struct UserArgs {
    user: String,
    mode: String,
    _unsused: String,
    real_name: String
}

impl RunCommand for UserArgs {
    fn run(
        self,
        _state: &mut crate::connection::state::ConnectionState,
        _writer: &mut std::io::BufWriter<std::net::TcpStream>,
    ) -> anyhow::Result<()> {
        dbg!(self);

        Ok(())
    }
}

