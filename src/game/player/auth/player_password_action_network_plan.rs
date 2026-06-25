use crate::game::player::{
    PlayerLoginNetworkPlan, PlayerPasswordActionOutcome, PlayerRegistrationNetworkPlan,
};
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerPasswordActionNetworkPlan;

impl PlayerPasswordActionNetworkPlan {
    pub fn plan(
        outcome: &PlayerPasswordActionOutcome,
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        match outcome {
            PlayerPasswordActionOutcome::Login(login) => {
                PlayerLoginNetworkPlan::plan(login, connection_id)
            }
            PlayerPasswordActionOutcome::Registration(registration) => {
                PlayerRegistrationNetworkPlan::plan(*registration, connection_id)
            }
            PlayerPasswordActionOutcome::ProfileUpdate(_) => Vec::new(),
        }
    }

    pub fn plan_all(
        outcomes: &[PlayerPasswordActionOutcome],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, connection_id))
            .collect()
    }
}
