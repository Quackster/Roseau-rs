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
