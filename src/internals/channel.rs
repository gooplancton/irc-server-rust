use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

type UserId = u64;

pub struct Channel {
    users: HashSet<UserId>,
}

impl Channel {
    pub fn new(first_user_id: Option<UserId>) -> Self {
        let users = if let Some(first_user_id) = first_user_id {
            HashSet::from([first_user_id])
        } else {
            HashSet::new()
        };

        Self { users }
    }
}

type ChannelName = String;

type ChannelsInner = HashMap<ChannelName, Channel>;

#[derive(Default, Clone)]
pub struct Channels {
    pub inner: Arc<RwLock<ChannelsInner>>,
}

impl Channels {
    pub fn _list(&self) -> Vec<String> {
        self.inner
            .read()
            .expect("dispatcher has panicked, aborting")
            .keys()
            .cloned()
            .collect()
    }

    pub fn get_channel_users(&self, name: &str) -> Option<HashSet<UserId>> {
        self.inner
            .read()
            .expect("dispatcher has panicked, aborting")
            .get(name)
            .map(|chan| chan.users.clone())
    }

    pub fn join_channel(&mut self, name: String, user_id: UserId) -> anyhow::Result<()> {
        let mut inner = self
            .inner
            .write()
            .expect("dispatcher has panicked, aborting");

        if let Some(existing_channel) = inner.get_mut(name.as_str()) {
            existing_channel.users.extend([user_id]);
            return Ok(());
        }

        let channel = Channel::new(Some(user_id));
        inner.insert(name, channel);

        Ok(())
    }

    pub fn leave_channel(&mut self, name: &str, user_id: &UserId) {
        let mut inner = self
            .inner
            .write()
            .expect("dispatcher has panicked, aborting");

        if let Some(existing_channel) = inner.get_mut(name) {
            existing_channel.users.remove(user_id);
        }
    }
}
