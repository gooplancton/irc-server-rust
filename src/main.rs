use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
};

use irc_parser::FromIRCString;

use commands::{handle_capabilities, handle_join, handle_nick, handle_user, Command};

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

        loop {
            let mut command_line = String::new();
            if reader.read_line(&mut command_line).is_err() || command_line.is_empty() {
                break;
            }

            dbg!(command_line.as_str());

            let command = Command::from_irc_string(&command_line);

            if let Err(err) = command {
                println!("{}", err);
                continue;
            }

            let messages = match command.unwrap() {
                Command::Join(args) => handle_join(args),
                Command::User(args) => handle_user(args),
                Command::Nick(args) => handle_nick(args),
                Command::Capabilities(args) => handle_capabilities(args)
            };

            messages.unwrap().into_iter().for_each(|message| {
                writer.write_all(message.as_bytes()).unwrap();
            });

            let _ = writer.flush();
        }
    }
}
