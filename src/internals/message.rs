#[derive(Debug)]
pub struct Message {
    pub header: Option<String>,
    pub recipient: String,
    pub content: String,
}

impl Message {
    pub fn private_message(from: Option<String>, to: String, content: String) -> Self {
        Self {
            header: from,
            recipient: to.clone(),
            content: format!("PRIVMSG {} :{}", to, content)
        }
    }
}

