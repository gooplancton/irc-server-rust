use std::{io::BufWriter, net::TcpStream, sync::mpsc::Sender};

use capabilities::CapabilitiesArgs;
use irc_parser::{FromIRCString, RunCommand};
use join::JoinArgs;
use mode::ModeArgs;
use nick::NickArgs;

use ping::PingArgs;
use privmsg::PrivMsgArgs;
use quit::QuitArgs;
use user::UserArgs;

use crate::{connection::state::ConnectionState, internals::server::Message};

mod capabilities;
mod join;
mod mode;
mod nick;
mod ping;
mod privmsg;
mod user;
mod quit;

pub trait RunCommand {
    fn run(
        self,
        state: &mut ConnectionState,
        writer: &mut BufWriter<TcpStream>,
        messages_tx: &mut Sender<Message>,
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

    #[command_name = "MODE"]
    Mode(ModeArgs),

    #[command_name = "PING"]
    Ping(PingArgs),

    #[command_name = "PRIVMSG"]
    PrivMsg(PrivMsgArgs),

    #[command_name = "QUIT"]
    Quit(QuitArgs)
}
