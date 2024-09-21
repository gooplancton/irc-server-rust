use internals::IRCServer;

mod commands;
mod consts;
mod internals;

fn main() {
    IRCServer::new().listen(None);
}
