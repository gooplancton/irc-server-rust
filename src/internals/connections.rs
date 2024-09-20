use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use thiserror::Error;

#[derive(Default, Clone)]
pub struct Connections {
    inner: Arc<Mutex<HashMap<String, TcpStream>>>,
}

#[derive(Debug, Error)]
pub enum ConnectionRegistrationError {
    #[error("could not acquire lock on connections")]
    PoisonError,
}

impl Connections {
    pub fn get_connection(&self, connection_id: &str) -> Option<TcpStream> {
        let connections = self.inner.lock().ok()?;
        let stream = connections.get(connection_id)?;

        Some(stream.try_clone().unwrap())
    }

    pub fn unregister_connection(
        &self,
        connection_id: &str,
    ) -> Result<(), ConnectionRegistrationError> {
        let mut connections = self
            .inner
            .lock()
            .map_err(|_| ConnectionRegistrationError::PoisonError)?;

        connections.remove(connection_id);

        Ok(())
    }

    pub fn register_connection(
        &mut self,
        connection_id: String,
        stream: TcpStream,
    ) -> Result<(), ConnectionRegistrationError> {
        let mut connections = self
            .inner
            .lock()
            .map_err(|_| ConnectionRegistrationError::PoisonError)?;

        connections.insert(connection_id, stream);

        Ok(())
    }

    pub fn reassign_connection(
        &mut self,
        old_connection_id: &str,
        new_connection_id: String,
    ) -> Result<(), ConnectionRegistrationError> {
        let mut connections = self
            .inner
            .lock()
            .map_err(|_| ConnectionRegistrationError::PoisonError)?;

        if let Some(stream) = connections.remove(old_connection_id) {
            connections.insert(new_connection_id, stream);
        }

        Ok(())
    }
}
