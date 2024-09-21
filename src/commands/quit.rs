use irc_parser::FromIRCString;

use crate::connection::state::RegistrationState;

use super::RunCommand;

#[derive(FromIRCString)]
pub struct QuitArgs {
    reason: Option<String>,
}

impl RunCommand for QuitArgs {
    fn run(
        self,
        state: &mut crate::connection::state::ConnectionState,
        _writer: &mut std::io::BufWriter<std::net::TcpStream>,
        _messages_tx: &mut std::sync::mpsc::Sender<crate::internals::Message>,
    ) -> anyhow::Result<()> {
        state.registration_state = RegistrationState::ReadyToBeUnregistered;

        Ok(())
    }
}
