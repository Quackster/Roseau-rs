use crate::protocol::NettyResponse;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerNetworkPlan {
    connection_id: i32,
    server_port: u16,
}

impl PlayerNetworkPlan {
    pub fn new(connection_id: i32, server_port: u16) -> Self {
        Self {
            connection_id,
            server_port,
        }
    }

    pub fn from_local_address(connection_id: i32, local_address: &str) -> Option<Self> {
        let port = local_address
            .rsplit(':')
            .next()
            .and_then(|value| value.parse::<u16>().ok())?;

        Some(Self::new(connection_id, port))
    }

    pub fn connection_id(&self) -> i32 {
        self.connection_id
    }

    pub fn server_port(&self) -> u16 {
        self.server_port
    }

    pub fn send_response(&self, mut response: NettyResponse) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id: self.connection_id,
            packet: response.get(),
        }
    }

    pub fn close(&self) -> PlayerNetworkEffect {
        PlayerNetworkEffect::CloseConnection {
            connection_id: self.connection_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_java_channel_local_address_port() {
        let plan = PlayerNetworkPlan::from_local_address(17, "/127.0.0.1:30001").unwrap();

        assert_eq!(plan.connection_id(), 17);
        assert_eq!(plan.server_port(), 30001);
    }

    #[test]
    fn rejects_missing_or_invalid_ports() {
        assert_eq!(PlayerNetworkPlan::from_local_address(1, "/127.0.0.1"), None);
        assert_eq!(
            PlayerNetworkPlan::from_local_address(1, "/127.0.0.1:not-a-port"),
            None
        );
        assert_eq!(
            PlayerNetworkPlan::from_local_address(1, "/127.0.0.1:70000"),
            None
        );
    }

    #[test]
    fn plans_write_and_close_effects() {
        let plan = PlayerNetworkPlan::new(22, 30002);

        assert_eq!(
            plan.send_response(NettyResponse::with_header("HELLO")),
            PlayerNetworkEffect::WriteResponse {
                connection_id: 22,
                packet: "#HELLO##".to_owned(),
            }
        );
        assert_eq!(
            plan.close(),
            PlayerNetworkEffect::CloseConnection { connection_id: 22 }
        );
    }
}
