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
    ) -> Self {
        Self {
            bind_ip: bind_ip.into(),
            raw_config_ip: raw_config_ip.into(),
            server_port,
            private_server_port,
            server_class_path: server_class_path.into(),
            database_engine,
            ports: ports.into(),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(bind_ip: &str, raw_config_ip: &str) -> ServerBootstrapPlan {
        ServerBootstrapPlan::new(
            bind_ip,
            raw_config_ip,
            37120,
            37119,
            "roseau::server::ServerHandler",
            DatabaseEngine::MySql,
            vec![37120, 37119],
        )
    }

    #[test]
    fn reports_numeric_config_ip_as_advertised_listen_address() {
        let plan = plan("127.0.0.1", "127.0.0.1");

        assert_eq!(plan.advertised_ip(Some("192.168.0.20")), "127.0.0.1");
        assert_eq!(
            plan.listen_status(true, Some("192.168.0.20")),
            RoseauStartupStatus::Listening {
                server_ip: "127.0.0.1".to_owned(),
                server_port: 37120,
            }
        );
    }

    #[test]
    fn reports_resolved_hostname_after_wildcard_bind() {
        let plan = plan("0.0.0.0", "roseau.local");

        assert_eq!(plan.advertised_ip(Some("10.0.0.25")), "10.0.0.25");
        assert_eq!(
            plan.listen_status(true, Some("10.0.0.25")).log_line(),
            "Server is listening on 10.0.0.25:37120"
        );
    }

    #[test]
    fn reports_listen_failure_on_primary_server_port() {
        let plan = plan("0.0.0.0", "roseau.local");

        assert_eq!(
            plan.listen_status(false, Some("10.0.0.25")),
            RoseauStartupStatus::ListenFailed { server_port: 37120 }
        );
    }

    #[test]
    fn reports_startup_status_from_listen_outcome() {
        let plan = plan("127.0.0.1", "127.0.0.1");
        let listen_plan =
            crate::server::ServerListenPlan::new(plan.bind_ip(), plan.ports().to_vec());

        assert_eq!(
            plan.listen_outcome_status(
                &ServerListenOutcome::success_for_plan(&listen_plan),
                Some("10.0.0.25")
            ),
            RoseauStartupStatus::Listening {
                server_ip: "127.0.0.1".to_owned(),
                server_port: 37120,
            }
        );
        assert_eq!(
            plan.listen_outcome_status(
                &ServerListenOutcome::failure_for_plan(&listen_plan, "127.0.0.1:37119"),
                Some("10.0.0.25")
            ),
            RoseauStartupStatus::ListenFailed { server_port: 37120 }
        );
    }
}
