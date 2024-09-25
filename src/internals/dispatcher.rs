use crate::consts::{ERR_NOSUCHCHANNEL, ERR_NOSUCHNICK};

use super::{
    channel::Channels, connection::Connections, message::MessageRecipient, user::Users, Message,
};
use bytes::Bytes;
use tokio::io::AsyncWriteExt;

pub struct Dispatcher {
    connections: Connections,
    channels: Channels,
    users: Users,
}

enum SendMessageError {
    NoSuchNick,
    NoSuchChannel,
}

impl Dispatcher {
    pub fn new(connections: Connections, channels: Channels, users: Users) -> Self {
        Self {
            connections,
            channels,
            users,
        }
    }

    pub async fn send_message(&mut self, message: &Message) {
        let send_res = match message.recipient {
            MessageRecipient::Channel(_) => self.send_message_to_channel(message).await,
            _ => self.send_message_to_user(message).await,
        };

        if let Err(error) = send_res {
            self.handle_send_error(message, error).await;
        }
    }

    async fn send_message_to_channel(&mut self, message: &Message) -> Result<(), SendMessageError> {
        let channel_name = match &message.recipient {
            MessageRecipient::Channel(name) => name,
            _ => unreachable!(),
        };

        let users = self.channels.get_channel_users(channel_name);

        if users.is_none() {
            return Err(SendMessageError::NoSuchChannel);
        }

        let sender_id = message.header.as_ref().and_then(|sender_nickname| {
            self.users
                .get_user_id(&String::from_utf8_lossy(sender_nickname))
        });

        for user_id in users.unwrap().drain() {
            if sender_id.is_some_and(|sender_id| sender_id == user_id) {
                continue;
            }

            let user_message = Message {
                header: message.header.clone(),
                recipient: MessageRecipient::UserId(user_id),
                content: message.content.clone(),
            };

            let _ = self.send_message_to_user(&user_message).await;
        }

        Ok(())
    }

    async fn send_message_to_user(&mut self, message: &Message) -> Result<(), SendMessageError> {
        let mut connections = self.connections.inner.lock().await;

        let recipient_user_id = match &message.recipient {
            MessageRecipient::Channel(_) => unreachable!(),
            MessageRecipient::UserId(id) => *id,
            MessageRecipient::Nickname(recipient_user_nickname) => self
                .users
                .get_user_id(recipient_user_nickname)
                .ok_or(SendMessageError::NoSuchNick)?,
        };

        let stream = connections.get_mut(&recipient_user_id);
        if stream.is_none() {
            // NOTE: user has disconnected, make it look like it does not exist for now
            return Err(SendMessageError::NoSuchNick);
        }

        let stream = stream.unwrap();
        let mut message_bytes: Vec<u8> = vec![];

        if let Some(header) = &message.header {
            message_bytes.push(b':');
            message_bytes.extend_from_slice(header);
            message_bytes.push(b' ');
        }

        message_bytes.extend_from_slice(&message.content);
        message_bytes.extend_from_slice(b"\r\n");

        let _ = stream.write_all(message_bytes.as_slice()).await;
        let _ = stream.flush().await;

        Ok(())
    }

    async fn handle_send_error(&mut self, message: &Message, error: SendMessageError) {
        let sender_nick = message
            .header
            .as_ref()
            .map(|header| String::from_utf8_lossy(header));

        if sender_nick.is_none() {
            println!("[handle_send_error] message does not have a sender, skipping...");
            return; // NOTE: this should never really happen
        }

        let sender_nick = sender_nick.unwrap();

        let sender_id = self.users.get_user_id(&sender_nick);
        if sender_id.is_none() {
            println!(
                "[handle_send_error] message has a sender nickname: {}, but not a user id, skipping...",
                sender_nick
            );
            return; // NOTE: this should never really happen
        }

        let sender_id = sender_id.unwrap();

        let content = match error {
            SendMessageError::NoSuchNick => {
                let recipient_nick = match &message.recipient {
                    MessageRecipient::Nickname(recipient_nick) => Some(recipient_nick),
                    _ => None,
                };

                if recipient_nick.is_none() {
                    dbg!(&message.recipient);
                    println!("[handle_send_error / NoSuchNick] message does not seem to have been sent to a nickname");
                    return; // NOTE: this should never really happen
                }

                Bytes::from(format!(
                    "{} {} {} :No such nickname",
                    ERR_NOSUCHNICK,
                    &sender_nick,
                    &recipient_nick.unwrap()
                ))
            }

            SendMessageError::NoSuchChannel => {
                let channel_name = match &message.recipient {
                    MessageRecipient::Channel(recipient_nick) => Some(recipient_nick),
                    _ => None,
                };

                if channel_name.is_none() {
                    dbg!(&message.recipient);
                    println!("[handle_send_error / NoSuchNick] message does not seem to have been sent to a channel");
                    return; // NOTE: this should never really happen
                }

                Bytes::from(format!(
                    "{} {} {} :No such channel",
                    ERR_NOSUCHCHANNEL,
                    &sender_nick,
                    &channel_name.unwrap()
                ))
            }
        };

        let message = Message {
            header: None,
            recipient: MessageRecipient::UserId(sender_id),
            content,
        };

        let _ = self.send_message_to_user(&message).await;
    }
}
