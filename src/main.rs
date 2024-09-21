use internals::IRCServer;

mod commands;
mod connection;
mod consts;
mod internals;

fn main() {
    IRCServer::new().listen(None);
}
