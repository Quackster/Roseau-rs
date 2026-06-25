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
