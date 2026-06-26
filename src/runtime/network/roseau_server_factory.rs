use crate::runtime::{BootstrapError, ServerBootstrapPlan};
use crate::server::{
    ServerConnectionHandler, ServerHandler, ServerListenPlan, TcpConnectionAcceptor,
    TcpServerRuntime,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauServerFactory {
    supported_handler_path: &'static str,
}

impl RoseauServerFactory {
    pub fn new() -> Self {
        Self {
            supported_handler_path: "roseau::server::ServerHandler",
        }
    }

    pub fn supported_handler_path(&self) -> &'static str {
        self.supported_handler_path
    }

    pub fn construct_handler(
        &self,
        plan: &ServerBootstrapPlan,
    ) -> Result<ServerHandler, BootstrapError> {
        if plan.server_class_path() != self.supported_handler_path {
            return Err(BootstrapError::UnsupportedServerHandler {
                class_path: plan.server_class_path().to_owned(),
            });
        }

        Ok(ServerHandler::new(plan.ports().to_vec(), plan.bind_ip()))
    }

    pub fn listen_plan(
        &self,
        plan: &ServerBootstrapPlan,
    ) -> Result<ServerListenPlan, BootstrapError> {
        let handler = self.construct_handler(plan)?;
        Ok(ServerListenPlan::from_handler(&handler))
    }

    pub fn construct_tcp_runtime(
        &self,
        plan: &ServerBootstrapPlan,
        log_connections: bool,
        log_packets: bool,
        first_connection_id: i32,
    ) -> Result<TcpServerRuntime, BootstrapError> {
        let handler = self.construct_handler(plan)?;
        let connection_handler = ServerConnectionHandler::new(log_connections, log_packets);
        let acceptor = TcpConnectionAcceptor::new(first_connection_id);

        Ok(TcpServerRuntime::new(handler, connection_handler, acceptor))
    }
}

impl Default for RoseauServerFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "roseau_server_factory_tests.rs"]
mod tests;
