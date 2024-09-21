use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use anyhow::{anyhow, bail};

pub struct Channel {
    name: String,
    users: Vec<String>,
}

impl Channel {
    pub fn new(name: String, first_user: Option<String>) -> Self {
        let users = if let Some(first_user) = first_user {
            vec![first_user]
        } else {
            vec![]
        };

        Self { name, users }
    }
}

type ChannelsInner = RwLock<HashMap<String, Channel>>;

#[derive(Default, Clone)]
pub struct Channels {
    pub inner: Arc<ChannelsInner>,
}

impl Channels {
    pub fn list(&self) -> anyhow::Result<Vec<String>> {
        self.inner
            .read()
            .map(|inner| inner.keys().cloned().collect::<Vec<String>>())
            .map_err(|_| anyhow!("failed to acquire lock"))
    }

    pub fn get_channel_users(&self, name: &str) -> anyhow::Result<Vec<String>> {
        self.inner
            .read()
            .map_err(|_| anyhow!("could not acquire lock"))?
            .get(name)
            .map(|chan| chan.users.clone())
            .ok_or(anyhow!("channel {} does not exist", &name))
    }

    pub fn create_channel(
        &mut self,
        name: String,
        first_user: Option<String>,
    ) -> anyhow::Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| anyhow!("failed to acquire lock"))?;

        if inner.contains_key(name.as_str()) {
            bail!("channel {} already exists", name);
        }

        let channel = Channel::new(name.clone(), first_user);
        inner.insert(name, channel);

        Ok(())
    }

    pub fn join_channel(&mut self, name: String, user: String) -> anyhow::Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| anyhow!("failed to acquire lock"))?;

        if let Some(existing_channel) = inner.get_mut(name.as_str()) {
            existing_channel.users.push(user);
            return Ok(());
        }

        let channel = Channel::new(name.clone(), Some(user));
        inner.insert(name, channel);

        Ok(())
    }

    pub fn leave_channel(&mut self, name: &str, user: &str) -> anyhow::Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| anyhow!("failed to acquire lock"))?;

        if let Some(existing_channel) = inner.get_mut(name) {
            let user_idx = existing_channel
                .users
                .iter()
                .position(|joined_user| joined_user == user);

            if let Some(user_idx) = user_idx {
                existing_channel.users.swap_remove(user_idx);
            }

            return Ok(());
        }

        Ok(())
    }
}
