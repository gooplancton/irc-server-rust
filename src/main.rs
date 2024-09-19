use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
};

use connection::state::ConnectionState;
use irc_parser::FromIRCString;

use commands::{Command, RunCommand};

mod commands;
mod connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6667").unwrap();

    for stream in listener.incoming() {
        if let Err(err) = stream {
            println!("error: {}", err);
            break;
        }

        let stream = stream.unwrap();
        let mut writer = BufWriter::new(stream.try_clone().unwrap());
        let mut reader = BufReader::new(stream);

        let mut connection_state = ConnectionState::new();

        loop {
            let mut command_line = String::new();
            if reader.read_line(&mut command_line).is_err() || command_line.is_empty() {
                break;
            }

            match Command::from_irc_string(&command_line) {
                Err(err) => println!("could not parse command {} due to {}", command_line, err),
                Ok(command) => {
                    let _ = command.run(&mut connection_state, &mut writer);
                    let _ = writer.flush();
                }
            }
        }
    }
}
