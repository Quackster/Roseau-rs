use crate::server::{ServerSocketBinder, TcpConnectionRuntime};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcpConnectionAcceptor {
    next_connection_id: i32,
    accepted_connections: usize,
    accept_errors: Vec<String>,
}

impl TcpConnectionAcceptor {
    pub fn new(first_connection_id: i32) -> Self {
        Self {
            next_connection_id: first_connection_id,
            accepted_connections: 0,
            accept_errors: Vec::new(),
        }
    }

    pub fn next_connection_id(&self) -> i32 {
        self.next_connection_id
    }

    pub fn accepted_connections(&self) -> usize {
        self.accepted_connections
    }

    pub fn accept_errors(&self) -> &[String] {
        &self.accept_errors
    }

    pub fn accept_one<B: ServerSocketBinder>(
        &mut self,
        binder: &B,
        listener_index: usize,
    ) -> Result<TcpConnectionRuntime, String> {
        let connection_id = self.next_connection_id;
        let stream = match binder.accept(listener_index) {
            Ok(stream) => stream,
            Err(error) => {
                self.accept_errors.push(error.clone());
                return Err(error);
            }
        };

        self.next_connection_id += 1;
        self.accepted_connections += 1;

        Ok(TcpConnectionRuntime::from_stream(connection_id, stream))
    }

    pub fn accept_one_nonblocking<B: ServerSocketBinder>(
        &mut self,
        binder: &B,
        listener_index: usize,
    ) -> Result<Option<TcpConnectionRuntime>, String> {
        let connection_id = self.next_connection_id;
        let stream = match binder.accept_nonblocking(listener_index) {
            Ok(Some(stream)) => stream,
            Ok(None) => return Ok(None),
            Err(error) => {
                self.accept_errors.push(error.clone());
                return Err(error);
            }
        };

        self.next_connection_id += 1;
        self.accepted_connections += 1;

        Ok(Some(TcpConnectionRuntime::from_stream(
            connection_id,
            stream,
        )))
    }
}

impl Default for TcpConnectionAcceptor {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
#[path = "tcp_connection_acceptor_tests.rs"]
mod tests;
