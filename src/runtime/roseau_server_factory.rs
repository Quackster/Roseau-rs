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
mod tests {
    use super::*;
    use crate::dao::mysql::DatabaseEngine;

    fn plan(class_path: &str) -> ServerBootstrapPlan {
        ServerBootstrapPlan::new(
            "127.0.0.1",
            "127.0.0.1",
            37120,
            37119,
            class_path,
            DatabaseEngine::MySql,
            vec![37120, 37119, 37125],
        )
    }

    #[test]
    fn constructs_rust_server_handler_from_bootstrap_plan() {
        let factory = RoseauServerFactory::new();
        let handler = factory
            .construct_handler(&plan(factory.supported_handler_path()))
            .unwrap();

        assert_eq!(handler.ip_address(), "127.0.0.1");
        assert_eq!(handler.ports(), &[37120, 37119, 37125]);
        assert!(handler.message_handler().contains_header("LOGIN"));
    }

    #[test]
    fn builds_listen_plan_from_constructed_handler() {
        let factory = RoseauServerFactory::new();
        let listen_plan = factory
            .listen_plan(&plan(factory.supported_handler_path()))
            .unwrap();

        assert_eq!(listen_plan.bind_ip(), "127.0.0.1");
        assert_eq!(
            listen_plan.bind_addresses(),
            vec![
                "127.0.0.1:37120".to_owned(),
                "127.0.0.1:37119".to_owned(),
                "127.0.0.1:37125".to_owned(),
            ]
        );
    }

    #[test]
    fn constructs_tcp_server_runtime_from_bootstrap_plan() {
        let factory = RoseauServerFactory::new();
        let runtime = factory
            .construct_tcp_runtime(&plan(factory.supported_handler_path()), true, false, 50)
            .unwrap();

        assert_eq!(runtime.server_handler().ip_address(), "127.0.0.1");
        assert_eq!(runtime.server_handler().ports(), &[37120, 37119, 37125]);
        assert_eq!(runtime.acceptor().next_connection_id(), 50);
        assert!(runtime.connections().is_empty());
    }

    #[test]
    fn rejects_unknown_rust_handler_path() {
        let result =
            RoseauServerFactory::new().construct_handler(&plan("roseau::server::OtherHandler"));
        let error = match result {
            Ok(_) => panic!("expected unsupported handler error"),
            Err(error) => error,
        };

        assert_eq!(
            error.to_string(),
            "unsupported server handler: roseau::server::OtherHandler"
        );
    }
}
