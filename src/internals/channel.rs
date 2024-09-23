use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::RwLock;

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

type ChannelsInner = RwLock<HashMap<ChannelName, Channel>>;

#[derive(Default, Clone)]
pub struct Channels {
    pub inner: Arc<ChannelsInner>,
}

impl Channels {
    pub async fn _list(&self) -> Vec<String> {
        self.inner.read().await.keys().cloned().collect()
    }

    pub async fn get_channel_users(&self, name: &str) -> Option<HashSet<UserId>> {
        self.inner
            .read()
            .await
            .get(name)
            .map(|chan| chan.users.clone())
    }

    pub async fn _create_channel(&mut self, name: String, first_user_id: Option<UserId>) {
        let mut inner = self.inner.write().await;

        if inner.contains_key(name.as_str()) {
            return;
        }

        let channel = Channel::new(first_user_id);
        inner.insert(name, channel);
    }

    pub async fn join_channel(&mut self, name: String, user_id: UserId) -> anyhow::Result<()> {
        let mut inner = self.inner.write().await;

        if let Some(existing_channel) = inner.get_mut(name.as_str()) {
            existing_channel.users.extend([user_id]);
            return Ok(());
        }

        let channel = Channel::new(Some(user_id));
        inner.insert(name, channel);

        Ok(())
    }

    pub async fn leave_channel(&mut self, name: &str, user_id: &UserId) {
        let mut inner = self.inner.write().await;

        if let Some(existing_channel) = inner.get_mut(name) {
            existing_channel.users.remove(user_id);
        }
    }
}
