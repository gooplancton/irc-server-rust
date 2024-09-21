use std::io::{BufWriter, Write};

use anyhow::anyhow;

use super::{channel::Channels, connections::Connections, Message};

pub struct Dispatcher {
    connections: Connections,
    channels: Channels,
}

impl Dispatcher {
    pub fn new(connections: Connections, channels: Channels) -> Self {
        Self {
            connections,
            channels,
        }
    }

    pub fn send_message(&self, message: Message) -> anyhow::Result<()> {
        if message.recipient.starts_with('#') | message.recipient.starts_with('&') {
            self.send_message_to_channel(message)
        } else {
            self.send_message_to_user(message)
        }
    }

    fn send_message_to_channel(&self, message: Message) -> anyhow::Result<()> {
        let channel_name = message.recipient.as_str();
        let users = self.channels.get_channel_users(channel_name)?;

        users.into_iter().for_each(|user| {
            if message
                .header
                .as_ref()
                .is_some_and(|sender| sender == &user)
            {
                return;
            }

            let user_message = Message {
                header: message.header.clone(),
                recipient: user.clone(),
                content: message.content.clone(),
            };

            let _ = self.send_message_to_user(user_message);
        });

        Ok(())
    }

    fn send_message_to_user(&self, message: Message) -> anyhow::Result<()> {
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
