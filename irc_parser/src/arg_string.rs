pub trait FromIRCString: Sized {
    fn from_irc_string(irc_string: &str) -> anyhow::Result<Self>;
}
