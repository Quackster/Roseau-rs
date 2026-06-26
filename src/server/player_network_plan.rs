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
#[path = "player_network_plan_tests.rs"]
mod tests;
