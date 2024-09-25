use capabilities::CapabilitiesArgs;
use irc_parser::{FromIRCString, RunCommand};
use join::JoinArgs;
use tokio::sync::mpsc::Sender;
use nick::NickArgs;
use ping::PingArgs;
use privmsg::PrivMsgArgs;
use quit::QuitArgs;
use user::UserArgs;

use crate::internals::{ConnectionState, Message};

mod capabilities;
mod join;
mod nick;
mod ping;
mod privmsg;
mod quit;
mod user;

#[derive(Default)]
pub struct CommandOutput {
    pub new_nickname: Option<String>,
    pub joined_channels: Option<Vec<String>>,
    pub left_channels: Option<Vec<String>>,
    pub disconnect: bool,
}

impl CommandOutput {
    pub fn join_channels(channels: Vec<String>) -> Self {
        Self {
            new_nickname: None,
            joined_channels: Some(channels),
            left_channels: None,
            disconnect: false,
        }
    }

    pub fn disconnect() -> Self {
        Self {
            new_nickname: None,
            joined_channels: None,
            left_channels: None, // TODO: leave channels when disconnecting?
            disconnect: true,
        }
    }

    pub fn rename(new_nickname: String) -> Self {
        Self {
            new_nickname: Some(new_nickname),
            joined_channels: None,
            left_channels: None,
            disconnect: false,
        }
    }
}

pub trait RunCommand {
    async fn run(
        self,
        state: &ConnectionState,
        outbox: &Sender<Message>,
    ) -> anyhow::Result<CommandOutput>;
}

#[derive(FromIRCString, RunCommand)]
#[command_list]
pub enum Command {
    #[command_name = "JOIN"]
    Join(JoinArgs),

    #[command_name = "CAP"]
    Capabilities(CapabilitiesArgs),

    #[command_name = "NICK"]
    Nick(NickArgs),

    #[command_name = "USER"]
    User(UserArgs),

    #[command_name = "PING"]
    Ping(PingArgs),

    #[command_name = "PRIVMSG"]
    PrivMsg(PrivMsgArgs),

    #[command_name = "QUIT"]
    Quit(QuitArgs),
}
