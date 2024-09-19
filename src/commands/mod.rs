use std::{io::BufWriter, net::TcpStream};

use capabilities::CapabilitiesArgs;
use irc_parser::{FromIRCString, RunCommand};
use join::JoinArgs;
use nick::NickArgs;

use user::UserArgs;

use crate::connection::state::ConnectionState;

mod capabilities;
mod join;
mod nick;
mod user;

pub trait RunCommand {
    fn run(
        self,
        state: &mut ConnectionState,
        writer: &mut BufWriter<TcpStream>,
    ) -> anyhow::Result<()>;
}

#[derive(FromIRCString, RunCommand)]
pub enum Command {
    #[command_name = "JOIN"]
    Join(JoinArgs),

    #[command_name = "CAP"]
    Capabilities(CapabilitiesArgs),

    #[command_name = "NICK"]
    Nick(NickArgs),

    #[command_name = "USER"]
    User(UserArgs),
}
