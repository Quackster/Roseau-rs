#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoseauStartupStatus {
    PreparingServer,
    DatabaseEngineRejected { engine: String },
    Listening { server_ip: String, server_port: u16 },
    ListenFailed { server_port: u16 },
}

impl RoseauStartupStatus {
    pub fn log_line(&self) -> String {
        match self {
            Self::PreparingServer => "Settting up server".to_owned(),
            Self::DatabaseEngineRejected { engine } => {
                format!("Unsupported database engine: {engine}")
            }
            Self::Listening {
                server_ip,
                server_port,
            } => format!("Server is listening on {server_ip}:{server_port}"),
            Self::ListenFailed { server_port } => format!(
                "Server could not listen on {server_port}:{server_port}, please double check everything is correct in icarus.properties"
            ),
        }
    }
}

#[cfg(test)]
#[path = "roseau_startup_status_tests.rs"]
mod tests;
