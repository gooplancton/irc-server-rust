use irc_parser::FromIRCString;

use crate::connection::state::ConnectionState;

use super::RunCommand;

#[derive(Debug, FromIRCString)]
pub struct NickArgs {
    nickname: String,
}

impl RunCommand for NickArgs {
    fn run(
        self,
        state: &mut ConnectionState,
        _writer: &mut std::io::BufWriter<std::net::TcpStream>,
    ) -> anyhow::Result<()> {
        state.nickname = Some(self.nickname);

        Ok(())
    }
}

