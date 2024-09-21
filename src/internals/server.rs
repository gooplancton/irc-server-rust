use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use irc_parser::FromIRCString;

use crate::commands::{Command, RunCommand};

use super::{
    channel::Channels,
    connection_state::{ChannelMembershipChange, ConnectionState, RegistrationState},
    connections::Connections,
    dispatcher::Dispatcher,
    message::Message,
};

pub struct IRCServer {
    channels: Channels,
    connections: Connections,
    messages_tx: Sender<Message>,
    messages_rx: Receiver<Message>,
}

impl IRCServer {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Message>();

        Self {
            channels: Channels::default(),
            connections: Connections::default(),
            messages_tx: tx,
            messages_rx: rx,
        }
    }

    pub fn listen(self, port: Option<usize>) {
        let addr = format!("127.0.0.1:{}", port.unwrap_or(6667));
        let listener = TcpListener::bind(addr).unwrap();

        let dispatcher_channels = self.channels.clone();
        let dispatcher_connections = self.connections.clone();
        thread::spawn(move || {
            let dispatcher = Dispatcher::new(
                dispatcher_connections,
                dispatcher_channels,
            );

            for message in self.messages_rx {
                let _ = dispatcher.send_message(message);
            }
        });

        for stream in listener.incoming() {
            if let Err(err) = stream {
                println!("error: {}", err);
                break;
            }

            let stream = stream.unwrap();
            let mut writer = BufWriter::new(stream.try_clone().unwrap());
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut messages_tx = self.messages_tx.clone();
            let mut handler_connections = self.connections.clone();
            let mut handler_channels = self.channels.clone();

            thread::spawn(move || {
                let mut connection_state = ConnectionState::new();

                loop {
                    let mut command_line = String::new();
                    if reader.read_line(&mut command_line).is_err() || command_line.is_empty() {
                        break;
                    }

                    dbg!(&command_line);
                    let res = Command::from_irc_string(&command_line).and_then(|command| {
                        command.run(&mut connection_state, &mut writer, &mut messages_tx)
                    });

                    if let Err(err) = res {
                        println!("error: {}", err);
                    }

                    let _ = writer.flush();

                    match connection_state.registration_state {
                        RegistrationState::ReadyToBeRegistered => {
                            let _ = handler_connections.register_connection(
                                connection_state.nickname.clone().unwrap(),
                                stream.try_clone().unwrap(),
                            );
                            connection_state.registration_state =
                                RegistrationState::AlreadyRegistered
                        }
                        RegistrationState::ReadyToBeUnregistered => {
                            let _ = handler_connections
                                .unregister_connection(connection_state.nickname.as_ref().unwrap());

                            return;
                        }
                        _ => {}
                    }

                    if connection_state.nickname.is_none() {
                        continue;
                    }

                    match connection_state.channel_membership_changes.take() {
                        None => {}
                        Some(changes) => {
                            let nickname = connection_state.nickname.as_ref().unwrap();
                            changes.into_iter().for_each(|change| match change {
                                ChannelMembershipChange::Joined(name) => {
                                    let res = handler_channels
                                        .join_channel(name.clone(), nickname.clone());
                                    if res.is_ok() {
                                        connection_state.joined_channels.push(name);
                                    }
                                }
                                ChannelMembershipChange::Left(name) => {
                                    let res =
                                        handler_channels.leave_channel(&name, nickname.as_str());
                                    if res.is_ok() {
                                        let channel_idx = connection_state
                                            .joined_channels
                                            .iter()
                                            .position(|channel_name| channel_name == &name);

                                        if let Some(channel_idx) = channel_idx {
                                            connection_state
                                                .joined_channels
                                                .swap_remove(channel_idx);
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            });
        }
    }
}
