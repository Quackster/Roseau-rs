use crate::game::room::RoomEntryOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEntryNetworkPlan;

impl RoomEntryNetworkPlan {
    pub fn plan(outcome: &RoomEntryOutcome, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        if let Some(packet) = outcome.flat_let_in() {
            return vec![Self::write(connection_id, packet.compose().get())];
        }

        outcome
            .error()
            .map(|packet| vec![Self::write(connection_id, packet.compose().get())])
            .unwrap_or_default()
    }

    pub fn plan_all(outcomes: &[RoomEntryOutcome], connection_id: i32) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, connection_id))
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
#[path = "room_entry_network_plan_tests.rs"]
mod tests;
