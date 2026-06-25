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
mod tests {
    use super::*;

    #[test]
    fn preserves_java_setup_status_line() {
        assert_eq!(
            RoseauStartupStatus::PreparingServer.log_line(),
            "Settting up server"
        );
    }

    #[test]
    fn formats_rejected_database_engine_line() {
        assert_eq!(
            RoseauStartupStatus::DatabaseEngineRejected {
                engine: "oracle".to_owned(),
            }
            .log_line(),
            "Unsupported database engine: oracle"
        );
    }

    #[test]
    fn formats_listen_result_lines_like_java_entrypoint() {
        assert_eq!(
            RoseauStartupStatus::Listening {
                server_ip: "127.0.0.1".to_owned(),
                server_port: 37120,
            }
            .log_line(),
            "Server is listening on 127.0.0.1:37120"
        );
        assert_eq!(
            RoseauStartupStatus::ListenFailed { server_port: 37120 }.log_line(),
            "Server could not listen on 37120:37120, please double check everything is correct in icarus.properties"
        );
    }
}
