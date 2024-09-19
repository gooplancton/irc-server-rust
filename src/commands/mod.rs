use capabilities::CapabilitiesArgs;
use irc_parser::FromIRCString;
use join::JoinArgs;
use nick::NickArgs;

pub use capabilities::handle_capabilities;
pub use join::handle_join;
pub use nick::handle_nick;
pub use user::handle_user;

use user::UserArgs;

mod capabilities;
mod join;
mod nick;
mod user;

#[derive(FromIRCString)]
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
