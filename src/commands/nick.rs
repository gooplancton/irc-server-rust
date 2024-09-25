use irc_parser::FromIRCString;
use tokio::sync::mpsc::Sender;

use crate::internals::{ConnectionState, Message};

use super::{CommandOutput, RunCommand};

#[derive(Debug, FromIRCString)]
pub struct NickArgs {
    nickname: String,
}

impl RunCommand for NickArgs {
    async fn run(
        self,
        _state: &ConnectionState,
        _outbox: Sender<Message>,
    ) -> anyhow::Result<CommandOutput> {
        Ok(CommandOutput::rename(self.nickname))
    }
}
