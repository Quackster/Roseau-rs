use crate::game::room::RoomUnitOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomUnitNetworkPlan;

impl RoomUnitNetworkPlan {
    pub fn plan(
        outcome: &RoomUnitOutcome,
        connection_id: i32,
        server_ip: &str,
        base_port: i32,
    ) -> Vec<PlayerNetworkEffect> {
        let mut effects = Vec::new();

        if let Some(packet) = outcome.all_units(server_ip, base_port) {
            effects.push(Self::write(connection_id, packet.compose().get()));
        }

        if let Some(packet) = outcome.unit_members_packet() {
            effects.push(Self::write(connection_id, packet.compose().get()));
        }

        effects
    }

    pub fn plan_all(
        outcomes: &[RoomUnitOutcome],
        connection_id: i32,
        server_ip: &str,
        base_port: i32,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, connection_id, server_ip, base_port))
            .collect()
    }

    fn write(connection_id: i32, packet: String) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id,
            packet,
        }
    }
}

#[cfg(test)]
#[path = "room_unit_network_plan_tests.rs"]
mod tests;
