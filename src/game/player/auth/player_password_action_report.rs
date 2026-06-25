use crate::game::player::{
    PlayerEffect, PlayerPasswordActionEffectPlan, PlayerPasswordActionNetworkPlan,
    PlayerPasswordActionOutcome,
};
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPasswordActionReport {
    outcomes: Vec<PlayerPasswordActionOutcome>,
    network_effects: Vec<PlayerNetworkEffect>,
    player_effects: Vec<PlayerEffect>,
}

impl PlayerPasswordActionReport {
    pub fn from_outcomes(
        outcomes: impl Into<Vec<PlayerPasswordActionOutcome>>,
        connection_id: i32,
    ) -> Self {
        let outcomes = outcomes.into();
        let network_effects = PlayerPasswordActionNetworkPlan::plan_all(&outcomes, connection_id);
        let player_effects = PlayerPasswordActionEffectPlan::plan_all(&outcomes);

        Self {
            outcomes,
            network_effects,
            player_effects,
        }
    }

    pub fn outcomes(&self) -> &[PlayerPasswordActionOutcome] {
        &self.outcomes
    }

    pub fn network_effects(&self) -> &[PlayerNetworkEffect] {
        &self.network_effects
    }

    pub fn player_effects(&self) -> &[PlayerEffect] {
        &self.player_effects
    }
}
