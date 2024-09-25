use bytes::Bytes;

use crate::consts::{ERR_NICKNAMEINUSE, RPL_WELCOME};

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
    pub fn nickname_unavailable(user_id: u64, nickname: &str, unavailable_nickname: &str) -> Self {
        let content = Bytes::from(format!(
            "{} {} {} :Nickname is already in use",
            ERR_NICKNAMEINUSE, nickname, unavailable_nickname
        ));

        Message {
            header: None,
            content,
            recipient: MessageRecipient::UserId(user_id),
        }
    }

    pub fn nickname_changed(
        old_nickname: Option<String>,
        new_nickname: &str,
        recipient: MessageRecipient,
    ) -> Self {
        Message {
            header: old_nickname.map(Bytes::from),
            content: Bytes::from(format!("NICK {}", new_nickname)),
            recipient,
        }
    }

    pub fn welcome(user_id: u64, nickname: &str) -> Self {
        let content = Bytes::from(format!(
            "{} {} :Welcome {}",
            RPL_WELCOME, nickname, nickname
        ));

        Message {
            header: None,
            content,
            recipient: MessageRecipient::UserId(user_id),
        }
    }
}
