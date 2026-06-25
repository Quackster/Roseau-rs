use crate::dao::mysql::DatabaseEngine;
use crate::runtime::RoseauStartupStatus;
use crate::server::ServerListenOutcome;
use crate::util::has_valid_ip_address;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerBootstrapPlan {
    bind_ip: String,
    raw_config_ip: String,
    server_port: u16,
    private_server_port: u16,
    server_class_path: String,
    database_engine: DatabaseEngine,
    ports: Vec<u16>,
    public_room_ports: Vec<(i32, u16)>,
}

impl ServerBootstrapPlan {
    pub fn new(
        bind_ip: impl Into<String>,
        raw_config_ip: impl Into<String>,
        server_port: u16,
        private_server_port: u16,
        server_class_path: impl Into<String>,
        database_engine: DatabaseEngine,
        ports: impl Into<Vec<u16>>,
        public_room_ports: impl Into<Vec<(i32, u16)>>,
    ) -> Self {
        Self {
            bind_ip: bind_ip.into(),
            raw_config_ip: raw_config_ip.into(),
            server_port,
            private_server_port,
            server_class_path: server_class_path.into(),
            database_engine,
            ports: ports.into(),
            public_room_ports: public_room_ports.into(),
        }
    }

    pub fn bind_ip(&self) -> &str {
        &self.bind_ip
    }

    pub fn raw_config_ip(&self) -> &str {
        &self.raw_config_ip
    }

    pub fn server_port(&self) -> u16 {
        self.server_port
    }

    pub fn private_server_port(&self) -> u16 {
        self.private_server_port
    }

    pub fn server_class_path(&self) -> &str {
        &self.server_class_path
    }

    pub fn database_engine(&self) -> DatabaseEngine {
        self.database_engine
    }

    pub fn ports(&self) -> &[u16] {
        &self.ports
    }

    pub fn public_room_ports(&self) -> &[(i32, u16)] {
        &self.public_room_ports
    }

    pub fn advertised_ip(&self, resolved_config_ip: Option<&str>) -> String {
        if has_valid_ip_address(&self.raw_config_ip) {
            self.raw_config_ip.clone()
        } else {
            resolved_config_ip.unwrap_or(&self.raw_config_ip).to_owned()
        }
    }

    pub fn listen_status(
        &self,
        listened: bool,
        resolved_config_ip: Option<&str>,
    ) -> RoseauStartupStatus {
        if listened {
            RoseauStartupStatus::Listening {
                server_ip: self.advertised_ip(resolved_config_ip),
                server_port: self.server_port,
            }
        } else {
            RoseauStartupStatus::ListenFailed {
                server_port: self.server_port,
            }
        }
    }

    pub fn listen_outcome_status(
        &self,
        outcome: &ServerListenOutcome,
        resolved_config_ip: Option<&str>,
    ) -> RoseauStartupStatus {
        self.listen_status(outcome.listened(), resolved_config_ip)
    }

    pub fn public_room_listening_lines(&self, resolved_config_ip: Option<&str>) -> Vec<String> {
        let server_ip = self.advertised_ip(resolved_config_ip);

        self.public_room_ports
            .iter()
            .map(|(room_id, port)| {
                format!("Public room {room_id} is listening on {server_ip}:{port}")
            })
            .collect()
    }
}

#[cfg(test)]
#[path = "server_bootstrap_plan_tests.rs"]
mod tests;
