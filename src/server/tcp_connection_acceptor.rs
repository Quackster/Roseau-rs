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
mod tests {
    use super::*;
    use std::io::Read;
    use std::net::TcpStream;
    use std::time::Duration;

    use crate::server::{
        ServerConnectionHandler, ServerHandler, ServerListenEffectExecutor, ServerListenPlan,
        StdTcpSocketBinder,
    };

    fn bound_binder() -> (StdTcpSocketBinder, std::net::SocketAddr) {
        let binder = StdTcpSocketBinder::new();
        let plan = ServerListenPlan::new("127.0.0.1", vec![0]);
        let mut executor = ServerListenEffectExecutor::new();
        let outcome = executor.execute_plan(&plan, &binder);

        assert!(outcome.listened());

        let address = binder.local_addresses().unwrap()[0];
        (binder, address)
    }

    #[test]
    fn accepts_connection_runtime_from_bound_listener() {
        let (binder, address) = bound_binder();
        let mut client = TcpStream::connect(address).unwrap();
        let mut acceptor = TcpConnectionAcceptor::new(40);
        client
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();

        let mut runtime = acceptor.accept_one(&binder, 0).unwrap();
        let mut server_handler = ServerHandler::new(vec![address.port()], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(false, false);

        runtime.open(&mut server_handler, &connection_handler);

        let mut bytes = [0; 8];
        client.read_exact(&mut bytes).unwrap();

        assert_eq!(runtime.connection_id(), 40);
        assert_eq!(&bytes, b"#HELLO##");
        assert_eq!(acceptor.next_connection_id(), 41);
        assert_eq!(acceptor.accepted_connections(), 1);
    }

    #[test]
    fn records_accept_errors_without_consuming_connection_id() {
        let binder = StdTcpSocketBinder::new();
        let mut acceptor = TcpConnectionAcceptor::new(12);

        let error = acceptor.accept_one(&binder, 0).unwrap_err();

        assert_eq!(error, "listener 0 is not bound");
        assert_eq!(
            acceptor.accept_errors(),
            &["listener 0 is not bound".to_owned()]
        );
        assert_eq!(acceptor.next_connection_id(), 12);
        assert_eq!(acceptor.accepted_connections(), 0);
    }

    #[test]
    fn nonblocking_accept_preserves_connection_id_when_idle() {
        let (binder, _address) = bound_binder();
        let mut acceptor = TcpConnectionAcceptor::new(12);

        assert!(acceptor
            .accept_one_nonblocking(&binder, 0)
            .unwrap()
            .is_none());
        assert_eq!(acceptor.next_connection_id(), 12);
        assert_eq!(acceptor.accepted_connections(), 0);
    }
}
