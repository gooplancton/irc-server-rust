use std::io::Write;

use irc_parser::FromIRCString;

use crate::connection::state::ConnectionState;

use super::RunCommand;

#[derive(FromIRCString)]
pub struct CapabilitiesArgs {
    subcommand: String,
}

impl RunCommand for CapabilitiesArgs {
    fn run(
        self,
        state: &mut ConnectionState,
        writer: &mut std::io::BufWriter<std::net::TcpStream>,
    ) -> anyhow::Result<()> {
        match self.subcommand.as_str() {
            "LS" => writer.write_all("CAP * LS :\r\n".as_bytes())?,
            "END" => {
                let nickname = state
                    .nickname
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("guest");

                let message = format!(
                    "001 {} :Michele è down, benvenuto a Radix Trading\r\n",
                    nickname
                );

                writer.write_all(message.as_bytes())?
            }
            _ => todo!(),
        };

        Ok(())
    }
}
