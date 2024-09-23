use std::{collections::HashMap, sync::Arc};

use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::RwLock;

type UserId = u64;

#[derive(Default, Clone)]
pub struct Connections {
    pub inner: Arc<RwLock<HashMap<UserId, OwnedWriteHalf>>>,
}

impl Connections {
    pub async fn unregister_connection(&self, connection_id: &UserId) {
        let mut connections = self.inner.write().await;
        connections.remove(connection_id);
    }

    pub async fn register_connection(&mut self, user_id: UserId, stream: OwnedWriteHalf) {
        let mut connections = self.inner.write().await;
        connections.insert(user_id, stream);
    }
}
