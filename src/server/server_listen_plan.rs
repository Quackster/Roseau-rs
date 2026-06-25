use crate::server::{ServerHandler, ServerListenEffect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerListenPlan {
    bind_ip: String,
    ports: Vec<u16>,
    pipeline: Vec<&'static str>,
}

impl ServerListenPlan {
    pub fn from_handler(handler: &ServerHandler) -> Self {
        Self::new(handler.ip_address(), handler.ports().to_vec())
    }

    pub fn new(bind_ip: impl Into<String>, ports: impl Into<Vec<u16>>) -> Self {
        Self {
            bind_ip: bind_ip.into(),
            ports: ports.into(),
            pipeline: vec!["encoder", "decoder", "handler"],
        }
    }

    pub fn bind_ip(&self) -> &str {
        &self.bind_ip
    }

    pub fn ports(&self) -> &[u16] {
        &self.ports
    }

    pub fn pipeline(&self) -> &[&'static str] {
        &self.pipeline
    }

    pub fn bind_addresses(&self) -> Vec<String> {
        self.ports
            .iter()
            .map(|port| format!("{}:{port}", self.bind_ip))
            .collect()
    }

    pub fn listen_effects(&self) -> Vec<ServerListenEffect> {
        let mut effects = vec![ServerListenEffect::CreateCachedWorkerPools];

        effects.extend(
            self.pipeline
                .iter()
                .copied()
                .map(|name| ServerListenEffect::InstallPipelineStage { name }),
        );
        effects.extend(
            self.bind_addresses()
                .into_iter()
                .map(|address| ServerListenEffect::BindAddress { address }),
        );

        effects
    }

    pub fn can_listen(&self) -> bool {
        !self.bind_ip.trim().is_empty() && !self.ports.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_listen_plan_from_handler_state() {
        let handler = ServerHandler::new(vec![30001, 30002], "127.0.0.1");

        let plan = ServerListenPlan::from_handler(&handler);

        assert_eq!(plan.bind_ip(), "127.0.0.1");
        assert_eq!(plan.ports(), &[30001, 30002]);
        assert_eq!(plan.pipeline(), &["encoder", "decoder", "handler"]);
        assert_eq!(
            plan.bind_addresses(),
            vec!["127.0.0.1:30001".to_owned(), "127.0.0.1:30002".to_owned()]
        );
        assert!(plan.can_listen());
    }

    #[test]
    fn rejects_empty_bind_inputs() {
        assert!(!ServerListenPlan::new("", vec![30001]).can_listen());
        assert!(!ServerListenPlan::new("127.0.0.1", Vec::<u16>::new()).can_listen());
    }

    #[test]
    fn plans_java_netty_listen_setup_order() {
        let plan = ServerListenPlan::new("127.0.0.1", vec![30001, 30002]);

        assert_eq!(
            plan.listen_effects(),
            vec![
                ServerListenEffect::CreateCachedWorkerPools,
                ServerListenEffect::InstallPipelineStage { name: "encoder" },
                ServerListenEffect::InstallPipelineStage { name: "decoder" },
                ServerListenEffect::InstallPipelineStage { name: "handler" },
                ServerListenEffect::BindAddress {
                    address: "127.0.0.1:30001".to_owned(),
                },
                ServerListenEffect::BindAddress {
                    address: "127.0.0.1:30002".to_owned(),
                },
            ]
        );
    }
}
