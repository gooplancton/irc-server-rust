#![allow(dead_code)]

use irc_parser::FromIRCString;

#[derive(Debug, FromIRCString)]
pub struct NickArgs {
    nickname: String
}

pub fn handle_nick(args: NickArgs) -> anyhow::Result<Vec<&'static str>> {
    dbg!(args);

    Ok(vec![])
}

