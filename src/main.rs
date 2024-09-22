use internals::IRCServer;

mod commands;
mod consts;
mod internals;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    IRCServer::new().listen(None).await
}
