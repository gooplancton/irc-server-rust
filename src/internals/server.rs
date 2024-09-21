use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use irc_parser::FromIRCString;

use crate::{
    commands::{Command, RunCommand},
    connection::state::{ConnectionState, RegistrationState},
};

use super::{connections::Connections, dispatcher::Dispatcher, message::Message};

pub struct IRCServer {
    connections: Connections,
    messages_tx: Sender<Message>,
    messages_rx: Receiver<Message>,
}

impl IRCServer {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Message>();

        Self {
            connections: Connections::default(),
            messages_tx: tx,
            messages_rx: rx,
        }
    }

    pub fn listen(self, port: Option<usize>) {
        let addr = format!("127.0.0.1:{}", port.unwrap_or(6667));
        let listener = TcpListener::bind(addr).unwrap();

        let dispatcher_connections = self.connections.clone();
        thread::spawn(move || {
            let dispatcher = Dispatcher::new(self.messages_rx, dispatcher_connections);
            dispatcher.start();
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

            thread::spawn(move || {
                let mut connection_state = ConnectionState::new();

                loop {
                    let mut command_line = String::new();
                    if reader.read_line(&mut command_line).is_err() || command_line.is_empty() {
                        break;
                    }

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
                }
            });
        }
    }
}
