use bytes::Bytes;

use crate::consts::{ERR_NICKNAMEINUSE, ERR_NOTREGISTERED, RPL_WELCOME};

#[derive(Debug)]
pub enum MessageRecipient {
    UserId(u64),
    Nickname(String),
    Channel(String),
}

impl MessageRecipient {
    pub fn from_string(s: String) -> Self {
        if s.starts_with('#') || s.starts_with('&') {
            Self::Channel(s)
        } else {
            Self::Nickname(s)
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub header: Option<Bytes>,
    pub recipient: MessageRecipient,
    pub content: Bytes,
}

impl Message {
    pub fn nickname_unavailable(
        recipient_user_id: u64,
        nickname: &str,
        unavailable_nickname: &str,
    ) -> Self {
        let content = Bytes::from(format!(
            "{} {} {} :Nickname is already in use",
            ERR_NICKNAMEINUSE, nickname, unavailable_nickname
        ));

        Message {
            header: None,
            content,
            recipient: MessageRecipient::UserId(recipient_user_id),
        }
    }

    pub fn nickname_changed(
        recipient: MessageRecipient,
        old_nickname: Option<String>,
        new_nickname: &str,
    ) -> Self {
        Message {
            header: old_nickname.map(Bytes::from),
            content: Bytes::from(format!("NICK {}", new_nickname)),
            recipient,
        }
    }

    pub fn welcome(recipient_user_id: u64, nickname: &str) -> Self {
        let content = Bytes::from(format!(
            "{} {} :Welcome {}",
            RPL_WELCOME, nickname, nickname
        ));

        Message {
            header: None,
            content,
            recipient: MessageRecipient::UserId(recipient_user_id),
        }
    }

    pub fn not_registered(recipient_user_id: u64, nickname: Option<&str>) -> Self {
        Message {
            header: None,
            recipient: MessageRecipient::UserId(recipient_user_id),
            content: Bytes::from(format!(
                "{} {} :You have not registered",
                ERR_NOTREGISTERED,
                nickname.unwrap_or("*")
            )),
        }
    }

    pub fn privmsg(recipient_string: String, sender: Bytes, text: &str) -> Self {
        let content = Bytes::from(format!("PRIVMSG {} :{}", &recipient_string, text));
        let recipient = MessageRecipient::from_string(recipient_string);

        Message {
            header: Some(sender),
            recipient,
            content,
        }
    }
}
