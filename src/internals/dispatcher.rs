use std::{
    io::{BufWriter, Write},
    sync::mpsc::Receiver,
};

use anyhow::anyhow;

use super::{connections::Connections, Message};

pub struct Dispatcher {
    messages_rx: Receiver<Message>,
    connections: Connections,
}

impl Dispatcher {
    pub fn new(messages_rx: Receiver<Message>, connections: Connections) -> Self {
        Self {
            messages_rx,
            connections,
        }
    }

    pub fn start(self) {
        while let Ok(message) = self.messages_rx.recv() {
            let res = self.send_message(message);
            if let Err(err) = res {
                println!("error sending message: {}", err);
            }
        }
    }

    fn send_message(&self, message: Message) -> anyhow::Result<()> {
        let stream = self
            .connections
            .get_connection(&message.recipient)
            .ok_or(anyhow!("no such connection: {}", message.recipient))?;

        let mut writer = BufWriter::new(stream);
        let message = if let Some(header) = message.header {
            format!(":{} {}\r\n", header, message.content)
        } else {
            format!("{}\r\n", message.content)
        };

        writer.write_all(message.as_bytes())?;
        writer.flush()?;

        Ok(())
    }
}
