use tokio::sync::mpsc::Sender;

use crate::commands::CommandOutput;

use super::{channel::Channels, message::MessageRecipient, user::{NicknameUnavailableError, Users}, Message};

pub struct ConnectionState {
    pub user_id: u64,
    pub nickname: Option<String>,
    pub joined_channels: Vec<String>,
}

impl ConnectionState {
    pub fn new(user_id: u64) -> Self {
        Self {
            user_id,
            nickname: None,
            joined_channels: vec![],
        }
    }

    pub async fn update(
        &mut self,
        channels: &mut Channels,
        users: &mut Users,
        outbox: &Sender<Message>,
        command_output: CommandOutput,
    ) {
        if let Some(new_nickname) = command_output.new_nickname {
            let rename_res = if let Some(ref previous_nickname) = self.nickname {
                users.rename_user(new_nickname.clone(), previous_nickname)
            } else {
                users.add_user(new_nickname.clone());
                Ok(())
            };

            match rename_res {
                Ok(_) => {
                    let old_nickname = std::mem::take(&mut self.nickname);
                    let _ = outbox
                        .send(Message::nickname_changed(
                            MessageRecipient::UserId(self.user_id),
                            old_nickname,
                            &new_nickname,
                        ))
                        .await; // TODO: perhaps should notify other people as well

                    self.nickname = Some(new_nickname);
                }
                Err(NicknameUnavailableError) => {
                    let _ = outbox
                        .send(Message::nickname_unavailable(
                            self.user_id,
                            self.nickname.as_ref().unwrap(),
                            &new_nickname,
                        ))
                        .await;
                }
            }
        }

        if let Some(joined_channels) = command_output.joined_channels {
            for joined_channel in joined_channels {
                let join_res = channels.join_channel(joined_channel.clone(), self.user_id);

                if join_res.is_ok() {
                    self.joined_channels.push(joined_channel);
                }
            }
        }

        if let Some(ref left_channels) = command_output.left_channels {
            for left_channel in left_channels {
                let idx = self
                    .joined_channels
                    .iter()
                    .position(|chan| chan == left_channel);

                if idx.is_none() {
                    continue;
                }

                self.joined_channels.swap_remove(idx.unwrap());
                channels.leave_channel(left_channel, &self.user_id);
            }
        }
    }
}
