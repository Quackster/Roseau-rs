use crate::game::player::PlayerRegistrationOutcome;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerRegistrationNetworkPlan;

impl PlayerRegistrationNetworkPlan {
    pub fn plan(
        _outcome: PlayerRegistrationOutcome,
        _connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        Vec::new()
    }

    pub fn plan_all(
        outcomes: &[PlayerRegistrationOutcome],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(*outcome, connection_id))
            .collect()
    }
}

#[cfg(test)]
#[path = "player_registration_network_plan_tests.rs"]
mod tests;
