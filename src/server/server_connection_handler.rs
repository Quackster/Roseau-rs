use crate::protocol::{ClientMessage, NettyRequest};
use crate::server::ServerConnectionEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConnectionHandler {
    log_connections: bool,
    log_packets: bool,
}

impl ServerConnectionHandler {
    pub fn new(log_connections: bool, log_packets: bool) -> Self {
        Self {
            log_connections,
            log_packets,
        }
    }

    pub fn channel_open(
        &self,
        connection_id: i32,
        remote_address: &str,
    ) -> Vec<ServerConnectionEffect> {
        let mut effects = vec![
            ServerConnectionEffect::SendHello { connection_id },
            ServerConnectionEffect::AddSession { connection_id },
        ];

        if self.log_connections {
            effects.push(ServerConnectionEffect::LogConnection {
                line: format!("[{connection_id}] Connection from {}", host(remote_address)),
            });
        }

        effects
    }

    pub fn channel_closed(
        &self,
        connection_id: i32,
        remote_address: &str,
    ) -> Vec<ServerConnectionEffect> {
        let mut effects = vec![
            ServerConnectionEffect::RemoveSession { connection_id },
            ServerConnectionEffect::DisposePlayer { connection_id },
        ];

        if self.log_connections {
            effects.push(ServerConnectionEffect::LogConnection {
                line: format!(
                    "[{connection_id}] Disconnection from {}",
                    host(remote_address)
                ),
            });
        }

        effects
    }

    pub fn message_received(
        &self,
        connection_id: i32,
        request: Option<NettyRequest>,
    ) -> Vec<ServerConnectionEffect> {
        let Some(request) = request else {
            return Vec::new();
        };

        let mut effects = Vec::new();

        if self.log_packets {
            effects.push(ServerConnectionEffect::LogPacket {
                line: Self::packet_log_line(connection_id, &request),
            });
        }

        effects.push(ServerConnectionEffect::DispatchRequest {
            connection_id,
            request,
        });

        effects
    }

    pub fn exception_caught(&self, connection_id: i32) -> Vec<ServerConnectionEffect> {
        vec![ServerConnectionEffect::CloseConnection { connection_id }]
    }

    pub fn packet_log_line(connection_id: i32, request: &NettyRequest) -> String {
        let header = request.get_header();

        if matches!(header, "LOGIN" | "INFORETRIEVE") && request.get_argument_amount() > 1 {
            format!(
                "[{connection_id}] Received: {} {}",
                header,
                request.get_argument(0).unwrap_or_default()
            )
        } else if header == "UPDATE" {
            format!("[{connection_id}] Received: {header}")
        } else {
            format!(
                "[{connection_id}] Received: {header} {}",
                request.get_message_body()
            )
        }
    }
}

fn host(address: &str) -> &str {
    let without_slash = address.strip_prefix('/').unwrap_or(address);
    without_slash.split(':').next().unwrap_or(without_slash)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(header: &str, body: &str) -> NettyRequest {
        NettyRequest::new(header, body)
    }

    #[test]
    fn open_and_close_emit_session_and_logging_effects() {
        let handler = ServerConnectionHandler::new(true, false);

        assert_eq!(
            handler.channel_open(7, "/127.0.0.1:30001"),
            vec![
                ServerConnectionEffect::SendHello { connection_id: 7 },
                ServerConnectionEffect::AddSession { connection_id: 7 },
                ServerConnectionEffect::LogConnection {
                    line: "[7] Connection from 127.0.0.1".to_owned(),
                },
            ]
        );
        assert_eq!(
            handler.channel_closed(7, "/127.0.0.1:30001"),
            vec![
                ServerConnectionEffect::RemoveSession { connection_id: 7 },
                ServerConnectionEffect::DisposePlayer { connection_id: 7 },
                ServerConnectionEffect::LogConnection {
                    line: "[7] Disconnection from 127.0.0.1".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn message_received_redacts_password_like_java_handler() {
        let handler = ServerConnectionHandler::new(false, true);
        let request = request("LOGIN", "alice secret");

        let effects = handler.message_received(9, Some(request.clone()));

        assert_eq!(
            effects,
            vec![
                ServerConnectionEffect::LogPacket {
                    line: "[9] Received: LOGIN alice".to_owned(),
                },
                ServerConnectionEffect::DispatchRequest {
                    connection_id: 9,
                    request,
                },
            ]
        );
    }

    #[test]
    fn packet_logging_handles_update_and_regular_messages() {
        assert_eq!(
            ServerConnectionHandler::packet_log_line(1, &request("UPDATE", "figure=hd")),
            "[1] Received: UPDATE"
        );
        assert_eq!(
            ServerConnectionHandler::packet_log_line(1, &request("TALK", "hello")),
            "[1] Received: TALK hello"
        );
    }

    #[test]
    fn ignores_missing_requests_and_closes_on_exception() {
        let handler = ServerConnectionHandler::new(false, true);

        assert!(handler.message_received(1, None).is_empty());
        assert_eq!(
            handler.exception_caught(1),
            vec![ServerConnectionEffect::CloseConnection { connection_id: 1 }]
        );
    }
}
