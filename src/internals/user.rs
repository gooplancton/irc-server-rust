use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hasher},
    sync::{Arc, RwLock},
    time::{self, UNIX_EPOCH},
};

use anyhow::bail;

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
type UsersInner = HashMap<Nickname, User>;

#[derive(Default, Clone)]
pub struct Users {
    pub inner: Arc<RwLock<UsersInner>>,
}

impl Users {
    pub fn rename_user(
        &mut self,
        new_nickname: String,
        previous_nickname: &str,
    ) -> anyhow::Result<()> {
        let mut inner = self
            .inner
            .write()
            .expect("dispatcher has panicked, aborting");

        if inner.contains_key(new_nickname.as_str()) {
            bail!("nickname {} alrady taken", new_nickname);
        }

        if let Some(user) = inner.remove(previous_nickname) {
            inner.insert(new_nickname, user);
        }

        Ok(())
    }

    pub fn add_user(&mut self, nickname: String) {
        let mut inner = self
            .inner
            .write()
            .expect("dispatcher has panicked, aborting");

        inner.insert(nickname.clone(), User::new());
    }

    pub fn _remove_user(&mut self, nickname: &str) {
        let mut inner = self
            .inner
            .write()
            .expect("dispatcher has panicked, aborting");

        inner.remove(nickname);
    }

    pub fn get_user_id(&self, nickname: &str) -> Option<u64> {
        let inner = self
            .inner
            .read()
            .expect("dispatcher has panicked, aborting");

        inner.get(nickname).map(|user| user.id)
    }
}
