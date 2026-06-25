use crate::game::player::PlayerLoginOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerLoginNetworkPlan;

impl PlayerLoginNetworkPlan {
    pub fn plan(outcome: &PlayerLoginOutcome, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        outcome
            .login_error()
            .map(|packet| {
                let mut response = packet.compose();
                vec![PlayerNetworkEffect::WriteResponse {
                    connection_id,
                    packet: response.get(),
                }]
            })
            .unwrap_or_default()
    }

    pub fn plan_all(
        outcomes: &[PlayerLoginOutcome],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, connection_id))
            .collect()
    }
}

#[cfg(test)]
#[path = "player_login_network_plan_tests.rs"]
mod tests;
