use irc_parser::{types::SpaceSeparatedList, FromIRCString};

#[derive(Default, FromIRCString)]
pub struct JoinArgs {
    channels: SpaceSeparatedList<String>,
}

pub fn handle_join(_command: JoinArgs) -> anyhow::Result<Vec<&'static str>> {
    Ok(vec!["JOIN\r\n"])
}

