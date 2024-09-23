use super::{
    channel::Channels, connections::Connections, message::MessageRecipient, user::Users, Message,
};
use anyhow::{anyhow, bail};
use tokio::io::AsyncWriteExt;

pub struct Dispatcher {
    connections: Connections,
    channels: Channels,
    users: Users,
}

impl Dispatcher {
    pub fn new(connections: Connections, channels: Channels, users: Users) -> Self {
        Self {
            connections,
            channels,
            users,
        }
    }

    pub async fn send_message(&mut self, message: Message) -> anyhow::Result<()> {
        match message.recipient {
            MessageRecipient::Channel(_) => self.send_message_to_channel(message).await,
            _ => self.send_message_to_user(message).await,
        }
    }

    async fn send_message_to_channel(&mut self, message: Message) -> anyhow::Result<()> {
        let channel_name = match message.recipient {
            MessageRecipient::Channel(name) => name,
            _ => unreachable!(),
        };

        let users = self.channels.get_channel_users(&channel_name).await;

        if users.is_none() {
            return Ok(());
        }

        let sender_id = if let Some(sender_nickname) = message.header.as_ref() {
            self.users.get_user_id(sender_nickname).await
        } else {
            None
        };

        for user_id in users.unwrap().drain() {
            if sender_id.is_some_and(|sender_id| sender_id == user_id) {
                continue;
            }

            let user_message = Message {
                header: message.header.clone(),
                recipient: MessageRecipient::UserId(user_id),
                content: message.content.clone(),
            };

            let _ = self.send_message_to_user(user_message).await;
        }

        Ok(())
    }

    async fn send_message_to_user(&mut self, message: Message) -> anyhow::Result<()> {
        let mut connections = self.connections.inner.write().await;

        let recipient_user_id = match message.recipient {
            MessageRecipient::Channel(_) => unreachable!(),
            MessageRecipient::UserId(id) => id,
            MessageRecipient::Nickname(recipient_user_nickname) => self
                .users
                .get_user_id(&recipient_user_nickname)
                .await
                .ok_or(anyhow!(
                    "no users with nickname {}",
                    recipient_user_nickname
                ))?,
        };

        let stream = connections.get_mut(&recipient_user_id);
        if stream.is_none() {
            bail!("user has disconnected")
        }

        let stream = stream.unwrap();

        let message = if let Some(header) = message.header {
            format!(":{} {}\r\n", header, message.content)
        } else {
            format!("{}\r\n", message.content)
        };

        stream.write_all(message.as_bytes()).await?;
        stream.flush().await?;

        Ok(())
    }
}
