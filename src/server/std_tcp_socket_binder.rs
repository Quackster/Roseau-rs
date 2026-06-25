use std::cell::RefCell;
use std::io::ErrorKind;
use std::net::{SocketAddr, TcpListener};

use crate::server::ServerSocketBinder;

#[derive(Debug, Default)]
pub struct StdTcpSocketBinder {
    listeners: RefCell<Vec<TcpListener>>,
}

impl StdTcpSocketBinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn listener_count(&self) -> usize {
        self.listeners.borrow().len()
    }

    pub fn local_addresses(&self) -> Result<Vec<SocketAddr>, String> {
        self.listeners
            .borrow()
            .iter()
            .map(|listener| listener.local_addr().map_err(|error| error.to_string()))
            .collect()
    }
}

impl ServerSocketBinder for StdTcpSocketBinder {
    fn bind(&self, address: &str) -> Result<(), String> {
        let listener = TcpListener::bind(address).map_err(|error| error.to_string())?;
        listener
            .set_nonblocking(true)
            .map_err(|error| error.to_string())?;
        self.listeners.borrow_mut().push(listener);
        Ok(())
    }

    fn accept(&self, listener_index: usize) -> Result<std::net::TcpStream, String> {
        let listeners = self.listeners.borrow();
        let listener = listeners
            .get(listener_index)
            .ok_or_else(|| format!("listener {listener_index} is not bound"))?;

        listener
            .accept()
            .map(|(stream, _)| stream)
            .map_err(|error| error.to_string())
    }

    fn accept_nonblocking(
        &self,
        listener_index: usize,
    ) -> Result<Option<std::net::TcpStream>, String> {
        let listeners = self.listeners.borrow();
        let listener = listeners
            .get(listener_index)
            .ok_or_else(|| format!("listener {listener_index} is not bound"))?;

        match listener.accept() {
            Ok((stream, _)) => Ok(Some(stream)),
            Err(error) if error.kind() == ErrorKind::WouldBlock => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }
}
