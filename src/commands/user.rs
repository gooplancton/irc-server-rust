#![allow(dead_code)]

use std::{
    io::{BufWriter, Write},
    net::TcpStream,
    sync::mpsc::Sender,
};

use anyhow::anyhow;
use irc_parser::FromIRCString;

use crate::{
    connection::state::{ConnectionState, RegistrationState},
    consts::RPL_WELCOME,
    internals::Message,
};

use super::RunCommand;

#[derive(Debug, FromIRCString)]
pub struct UserArgs {
    username: String,
    _unused1: String,
    _unused2: String,
    realname: String,
}

impl RunCommand for UserArgs {
    fn run(
        self,
        state: &mut ConnectionState,
        writer: &mut BufWriter<TcpStream>,
        _messages_tx: &mut Sender<Message>,
    ) -> anyhow::Result<()> {
        state.username = Some(self.username);

        let nickname = state
            .nickname
            .as_deref()
            .ok_or(anyhow!("nickname must be known at this point"))?;

        let message = format!(
            "{} {} :Welcome {}\r\n",
            RPL_WELCOME, nickname, nickname
        );

        writer.write_all(message.as_bytes())?;

        state.registration_state = RegistrationState::ReadyToBeRegistered;

        Ok(())
    }
}
