use std::net::TcpStream;

use crate::messages::IncomingContext;
use crate::protocol::DecodeError;
use crate::server::{
    PlayerNetworkEffect, PlayerNetworkEffectExecutor, ServerConnectionDriver,
    ServerConnectionEffectExecutor, ServerConnectionHandler, ServerHandler, TcpPlayerNetwork,
};

#[derive(Debug)]
pub struct TcpConnectionRuntime {
    driver: ServerConnectionDriver,
    network: TcpPlayerNetwork,
    effect_executor: ServerConnectionEffectExecutor,
    network_effect_executor: PlayerNetworkEffectExecutor,
    applied_network_effects: usize,
}

impl TcpConnectionRuntime {
    pub fn new(connection_id: i32, stream: TcpStream, remote_address: impl Into<String>) -> Self {
        Self {
            driver: ServerConnectionDriver::new(connection_id, remote_address),
            network: TcpPlayerNetwork::new(connection_id, stream),
            effect_executor: ServerConnectionEffectExecutor::new(),
            network_effect_executor: PlayerNetworkEffectExecutor::new(),
            applied_network_effects: 0,
        }
    }

    pub fn from_stream(connection_id: i32, stream: TcpStream) -> Self {
        let remote_address = stream
            .peer_addr()
            .map(|address| address.to_string())
            .unwrap_or_else(|_| String::new());

        Self::new(connection_id, stream, remote_address)
    }

    pub fn with_context(mut self, context: IncomingContext) -> Self {
        self.driver = self.driver.with_context(context);
        self
    }

    pub fn connection_id(&self) -> i32 {
        self.driver.connection_id()
    }

    pub fn context(&self) -> &IncomingContext {
        self.driver.context()
    }

    pub fn effect_executor(&self) -> &ServerConnectionEffectExecutor {
        &self.effect_executor
    }

    pub fn network(&self) -> &TcpPlayerNetwork {
        &self.network
    }

    pub fn network_effect_executor(&self) -> &PlayerNetworkEffectExecutor {
        &self.network_effect_executor
    }

    pub fn set_read_timeout(&self, timeout: Option<std::time::Duration>) -> Result<(), String> {
        self.network.set_read_timeout(timeout)
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> Result<(), String> {
        self.network.set_nonblocking(nonblocking)
    }

    pub fn open(
        &mut self,
        server_handler: &mut ServerHandler,
        connection_handler: &ServerConnectionHandler,
    ) {
        self.driver.open(
            server_handler,
            connection_handler,
            &mut self.effect_executor,
        );
        self.apply_pending_network_effects();
    }

    pub fn read_bytes(
        &mut self,
        bytes: impl AsRef<[u8]>,
        server_handler: &mut ServerHandler,
        connection_handler: &ServerConnectionHandler,
    ) -> Result<(), DecodeError> {
        let result = self.driver.read_bytes(
            bytes,
            server_handler,
            connection_handler,
            &mut self.effect_executor,
        );
        self.apply_pending_network_effects();
        result
    }

    pub fn read_from_network(
        &mut self,
        max_bytes: usize,
        server_handler: &mut ServerHandler,
        connection_handler: &ServerConnectionHandler,
    ) -> Result<usize, String> {
        let mut buffer = vec![0; max_bytes];
        let bytes_read = self.network.read_available(&mut buffer)?;

        if bytes_read == 0 {
            self.close(server_handler, connection_handler);
            return Ok(0);
        }

        self.read_bytes(&buffer[..bytes_read], server_handler, connection_handler)
            .map_err(|error| error.to_string())?;

        Ok(bytes_read)
    }

    pub fn read_from_network_nonblocking(
        &mut self,
        max_bytes: usize,
        server_handler: &mut ServerHandler,
        connection_handler: &ServerConnectionHandler,
    ) -> Result<Option<usize>, String> {
        let mut buffer = vec![0; max_bytes];
        let Some(bytes_read) = self.network.read_available_nonblocking(&mut buffer)? else {
            return Ok(None);
        };

        if bytes_read == 0 {
            self.close(server_handler, connection_handler);
            return Ok(Some(0));
        }

        self.read_bytes(&buffer[..bytes_read], server_handler, connection_handler)
            .map_err(|error| error.to_string())?;

        Ok(Some(bytes_read))
    }

    pub fn close(
        &mut self,
        server_handler: &mut ServerHandler,
        connection_handler: &ServerConnectionHandler,
    ) {
        self.driver.close(
            server_handler,
            connection_handler,
            &mut self.effect_executor,
        );
        self.apply_pending_network_effects();
    }

    pub fn apply_network_effect(&mut self, effect: PlayerNetworkEffect) -> bool {
        if effect.connection_id() != self.connection_id() {
            return false;
        }

        self.network_effect_executor
            .apply(&mut self.network, effect);
        true
    }

    fn apply_pending_network_effects(&mut self) {
        let effects =
            self.effect_executor.network_effects()[self.applied_network_effects..].to_vec();
        self.applied_network_effects += effects.len();
        self.network_effect_executor
            .apply_all(&mut self.network, effects);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::Shutdown;
    use std::net::TcpListener;
    use std::time::Duration;

    use crate::messages::IncomingCommand;
    use crate::server::PlayerNetwork;

    fn connected_runtime() -> (TcpConnectionRuntime, TcpStream) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let client = TcpStream::connect(address).unwrap();
        let (server_stream, _) = listener.accept().unwrap();

        (TcpConnectionRuntime::from_stream(15, server_stream), client)
    }

    #[test]
    fn opens_tcp_connection_and_writes_hello_packet() {
        let (mut runtime, mut client) = connected_runtime();
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(true, false);
        client
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();

        runtime.open(&mut server_handler, &connection_handler);

        let mut bytes = [0; 8];
        client.read_exact(&mut bytes).unwrap();

        assert_eq!(&bytes, b"#HELLO##");
        assert!(server_handler.session_manager().has_session(15));
        assert_eq!(runtime.connection_id(), 15);
        assert_eq!(
            runtime.effect_executor().connection_logs(),
            &["[15] Connection from 127.0.0.1".to_owned()]
        );
    }

    #[test]
    fn dispatches_complete_tcp_payloads_through_connection_driver() {
        let (runtime, _client) = connected_runtime();
        let mut runtime = runtime.with_context(IncomingContext::new().in_room(true));
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(false, true);

        runtime
            .read_bytes(b"0010CHAT hello", &mut server_handler, &connection_handler)
            .unwrap();

        assert_eq!(
            runtime.context().commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::Talk {
                    mode: "CHAT".to_owned(),
                    message: "hello".to_owned(),
                }
            ]
        );
        assert_eq!(
            runtime.effect_executor().packet_logs(),
            &["[15] Received: CHAT hello".to_owned()]
        );
    }

    #[test]
    fn closes_tcp_network_on_decode_error() {
        let (mut runtime, _client) = connected_runtime();
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(false, false);

        let error = runtime
            .read_bytes(b"BAD!CHAT hello", &mut server_handler, &connection_handler)
            .unwrap_err();

        assert_eq!(error, DecodeError::InvalidLength);
        assert!(runtime.network().is_closed());
    }

    #[test]
    fn reads_from_tcp_network_and_dispatches_complete_frames() {
        let (runtime, mut client) = connected_runtime();
        let mut runtime = runtime.with_context(IncomingContext::new().in_room(true));
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(false, true);
        runtime
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();

        client.write_all(b"0010CHAT hello").unwrap();
        let bytes_read = runtime
            .read_from_network(64, &mut server_handler, &connection_handler)
            .unwrap();

        assert_eq!(bytes_read, 14);
        assert_eq!(
            runtime.context().commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::Talk {
                    mode: "CHAT".to_owned(),
                    message: "hello".to_owned(),
                }
            ]
        );
    }

    #[test]
    fn nonblocking_read_reports_idle_without_closing_connection() {
        let (mut runtime, _client) = connected_runtime();
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(false, false);

        runtime.set_nonblocking(true).unwrap();
        let bytes_read = runtime
            .read_from_network_nonblocking(64, &mut server_handler, &connection_handler)
            .unwrap();

        assert_eq!(bytes_read, None);
        assert!(!runtime.network().is_closed());
    }

    #[test]
    fn eof_from_tcp_network_closes_connection_lifecycle() {
        let (mut runtime, client) = connected_runtime();
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(false, false);
        runtime.open(&mut server_handler, &connection_handler);
        runtime
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();

        client.shutdown(Shutdown::Write).unwrap();
        let bytes_read = runtime
            .read_from_network(64, &mut server_handler, &connection_handler)
            .unwrap();

        assert_eq!(bytes_read, 0);
        assert!(!server_handler.session_manager().has_session(15));
    }

    #[test]
    fn close_removes_session_without_replaying_prior_network_effects() {
        let (mut runtime, mut client) = connected_runtime();
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(false, false);
        client
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();

        runtime.open(&mut server_handler, &connection_handler);
        runtime.close(&mut server_handler, &connection_handler);

        let mut bytes = Vec::new();
        let _ = client.read_to_end(&mut bytes);

        assert_eq!(bytes, b"#HELLO##");
        assert!(!server_handler.session_manager().has_session(15));
        assert!(!runtime.network().is_closed());
    }

    #[test]
    fn skips_network_effects_for_other_connections() {
        let (mut runtime, _client) = connected_runtime();
        let applied =
            runtime.apply_network_effect(crate::server::PlayerNetworkEffect::CloseConnection {
                connection_id: 99,
            });

        assert!(!applied);
        assert!(!runtime.network().is_closed());
        assert!(runtime
            .network_effect_executor()
            .skipped_effects()
            .is_empty());
    }

    #[test]
    fn applies_matching_external_network_effect() {
        let (mut runtime, _client) = connected_runtime();

        let applied =
            runtime.apply_network_effect(crate::server::PlayerNetworkEffect::CloseConnection {
                connection_id: 15,
            });

        assert!(applied);
        assert!(runtime.network().is_closed());
    }
}
