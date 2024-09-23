use std::{collections::HashMap, sync::Arc};

use tokio::{net::tcp::OwnedWriteHalf, sync::Mutex};

type UserId = u64;

type ConnectionsInner = HashMap<UserId, OwnedWriteHalf>;

#[derive(Default, Clone)]
pub struct Connections {
    pub inner: Arc<Mutex<ConnectionsInner>>,
}

impl Connections {
    pub async fn unregister_connection(&self, connection_id: &UserId) {
        let mut connections = self.inner.lock().await;

        connections.remove(connection_id);
    }

    pub async fn register_connection(&mut self, user_id: UserId, stream: OwnedWriteHalf) {
        let mut connections = self.inner.lock().await;

        connections.insert(user_id, stream);
    }
}
