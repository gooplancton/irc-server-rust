use std::io::Write;
use std::{io::BufWriter, net::TcpStream, sync::mpsc::Sender};

use irc_parser::{types::SpaceSeparatedList, FromIRCString};

use crate::internals::connection_state::ChannelMembershipChange;
use crate::internals::{ConnectionState, Message};

use super::RunCommand;

#[derive(Default, FromIRCString)]
pub struct JoinArgs {
    channels: SpaceSeparatedList<String>,
}

impl RunCommand for JoinArgs {
    fn run(
        self,
        state: &mut ConnectionState,
        writer: &mut BufWriter<TcpStream>,
        _messages_tx: &mut Sender<Message>,
    ) -> anyhow::Result<()> {
        let membership_changes = self
            .channels
            .values
            .into_iter()
            .map(ChannelMembershipChange::Joined)
            .collect();

        state.channel_membership_changes = Some(membership_changes);

        writer.write_all("JOIN\r\n".as_bytes())?;

        Ok(())
    }
}
