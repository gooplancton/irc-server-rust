use std::str::FromStr;

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
    pub header: Option<String>,
    pub recipient: MessageRecipient,
    pub content: String,
}
