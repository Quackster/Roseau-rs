use crate::game::player::FindUserOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FindUserNetworkPlan;

impl FindUserNetworkPlan {
    pub fn plan(outcome: &FindUserOutcome, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        if let Some(packet) = outcome.member_info() {
            return vec![Self::write(connection_id, packet.compose().get())];
        }

        outcome
            .no_such_user()
            .map(|packet| vec![Self::write(connection_id, packet.compose().get())])
            .unwrap_or_default()
    }

    pub fn plan_all(outcomes: &[FindUserOutcome], connection_id: i32) -> Vec<PlayerNetworkEffect> {
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
#[path = "find_user_network_plan_tests.rs"]
mod tests;
