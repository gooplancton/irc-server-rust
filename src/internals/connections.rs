use std::{collections::HashMap, sync::Arc};

use tokio::net::tcp::{OwnedWriteHalf, WriteHalf};
use tokio::sync::{Mutex, RwLock};

type UserId = u64;

#[derive(Default, Clone)]
pub struct Connections {
    pub inner: Arc<RwLock<HashMap<UserId, Mutex<OwnedWriteHalf>>>>,
}

impl Connections {
    // pub async fn get_connection(&self, connection_id: &str) -> Option<&TcpStream> {
    //     let connections = self.inner.read().await;
    //     let stream = connections.get(connection_id)?;
    //
    //     Some(stream)
    // }

    pub async fn unregister_connection(&self, connection_id: &UserId) {
        let mut connections = self.inner.write().await;
        connections.remove(connection_id);
    }

    pub async fn register_connection(&mut self, user_id: UserId, stream: Mutex<OwnedWriteHalf>) {
        let mut connections = self.inner.write().await;
        connections.insert(user_id, stream);
    }

    pub async fn reassign_connection(
        &mut self,
        old_connection_id: &UserId,
        new_connection_id: UserId,
    ) {
        let mut connections = self.inner.write().await;
        if let Some(stream) = connections.remove(old_connection_id) {
            connections.insert(new_connection_id, stream);
        }
    }
}
