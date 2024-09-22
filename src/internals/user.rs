use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hasher},
    sync::Arc,
    time::{self, UNIX_EPOCH},
};

use anyhow::bail;
use tokio::sync::RwLock;

pub struct User {
    pub id: u64,
}

impl User {
    pub fn new() -> Self {
        let mut hasher = DefaultHasher::new();
        let timestamp = time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        hasher.write_u64(timestamp);
        let id = hasher.finish();

        Self { id }
    }
}

type Nickname = String;
type UsersInner = RwLock<HashMap<Nickname, User>>;

#[derive(Default, Clone)]
pub struct Users {
    pub inner: Arc<UsersInner>,
}

impl Users {
    pub async fn rename_user(
        &mut self,
        new_nickname: String,
        previous_nickname: &str,
    ) -> anyhow::Result<()> {
        let mut inner = self.inner.write().await;

        if inner.contains_key(new_nickname.as_str()) {
            bail!("nickname {} alrady taken", new_nickname);
        }

        if let Some(user) = inner.remove(previous_nickname) {
            inner.insert(new_nickname, user);
        }

        Ok(())
    }

    pub async fn add_user(&mut self, nickname: String) -> anyhow::Result<()> {
        let mut inner = self.inner.write().await;
        inner.insert(nickname.clone(), User::new());

        Ok(())
    }

    pub async fn remove_user(&mut self, nickname: &str) {
        let mut inner = self.inner.write().await;
        inner.remove(nickname);
    }

    pub async fn get_user_id(&self, nickname: &str) -> Option<u64> {
        self.inner.read().await.get(nickname).map(|user| user.id)
    }
}
