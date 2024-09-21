use irc_parser::FromIRCString;

use crate::internals::connection_state::RegistrationState;

use crate::internals::{ConnectionState, Message};

use super::RunCommand;

#[derive(FromIRCString)]
pub struct QuitArgs {
    reason: Option<String>,
}

impl RunCommand for QuitArgs {
    fn run(
        self,
        state: &mut ConnectionState,
        _writer: &mut std::io::BufWriter<std::net::TcpStream>,
        _messages_tx: &mut std::sync::mpsc::Sender<crate::internals::Message>,
    ) -> anyhow::Result<()> {
        state.registration_state = RegistrationState::ReadyToBeUnregistered;

        Ok(())
    }
}
