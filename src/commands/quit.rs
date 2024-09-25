use crate::internals::{ConnectionState, Message};
use irc_parser::FromIRCString;
use tokio::sync::mpsc::Sender;

use super::{CommandOutput, RunCommand};

#[derive(FromIRCString)]
pub struct QuitArgs {
    _reason: Option<String>,
}

impl RunCommand for QuitArgs {
    async fn run(
        self,
        _state: &ConnectionState,
        _outbox: Sender<Message>,
    ) -> anyhow::Result<CommandOutput> {
        // TODO: send disconnection message

        Ok(CommandOutput::disconnect())
    }
}
