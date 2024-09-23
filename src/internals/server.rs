use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use irc_parser::FromIRCString;

use crate::commands::{Command, RunCommand};

use super::dispatcher::Dispatcher;
use super::user::{User, Users};
use super::{
    channel::Channels, connection_state::ConnectionState, connections::Connections,
    message::Message,
};

pub struct IRCServer {
    channels: Channels,
    connections: Connections,
    users: Users,
    messages_tx: Sender<Message>,
    messages_rx: Receiver<Message>,
}

impl IRCServer {
    pub fn new() -> Self {
        let (tx, rx) = channel::<Message>(1024);

        Self {
            channels: Channels::default(),
            connections: Connections::default(),
            users: Users::default(),
            messages_tx: tx,
            messages_rx: rx,
        }
    }

    pub async fn listen(mut self, port: Option<usize>) -> anyhow::Result<()> {
        let addr = format!("127.0.0.1:{}", port.unwrap_or(6667));
        let listener = TcpListener::bind(addr).await.unwrap();

        let dispatcher_channels = self.channels.clone();
        let dispatcher_connections = self.connections.clone();
        let dispatcher_users = self.users.clone();
        tokio::spawn(async move {
            let mut dispatcher = Dispatcher::new(
                dispatcher_connections,
                dispatcher_channels,
                dispatcher_users,
            );

            while let Some(message) = self.messages_rx.recv().await {
                let _ = dispatcher.send_message(&message).await;
            }
        });

        loop {
            let (stream, _) = listener.accept().await?;

            let messages_tx = self.messages_tx.clone();
            let mut connections = self.connections.clone();
            let mut channels = self.channels.clone();
            let mut users = self.users.clone();

            tokio::spawn(async move {
                let (read_half, write_half) = stream.into_split();
                let mut reader = BufReader::new(read_half);

                let user = User::new();
                let mut connection_state = ConnectionState::new(user.id);

                connections.register_connection(user.id, write_half).await;

                let user_id = loop {
                    let mut command_line = String::new();
                    let read_res = reader.read_line(&mut command_line).await;
                    if let Err(err) = read_res {
                        eprint!("error reading command: {}", err);
                        break connection_state.user_id;
                    }

                    print!("received command: {}", &command_line);

                    let command = match Command::from_irc_string(&command_line) {
                        Ok(command) => command,
                        Err(err) => {
                            eprintln!("error parsing command: {}", err);
                            continue;
                        }
                    };

                    let output = command.run(&connection_state, messages_tx.clone()).await;
                    if let Err(err) = output {
                        eprintln!("error executing command: {}", err);
                        continue;
                    }

                    let output = output.unwrap();
                    let disconnect_after_update = output.disconnect;

                    connection_state
                        .update(&mut channels, &mut users, output)
                        .await;

                    if disconnect_after_update {
                        break connection_state.user_id;
                    }
                };

                connections.unregister_connection(&user_id).await;
            });
        }
    }
}
