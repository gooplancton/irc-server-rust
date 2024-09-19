#![allow(dead_code)]

use irc_parser::FromIRCString;

#[derive(Debug, FromIRCString)]
pub struct UserArgs {
    user: String,
    mode: String,
    _unsused: String,
    real_name: String
}

pub fn handle_user(args: UserArgs) -> anyhow::Result<Vec<&'static str>> {
    dbg!(args);

    Ok(vec![])
}

