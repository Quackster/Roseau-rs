use crate::game::navigator::NavigatorSearchOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NavigatorSearchNetworkPlan;

impl NavigatorSearchNetworkPlan {
    pub fn plan(outcome: &NavigatorSearchOutcome, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        let mut response = outcome.busy_flat_results().compose();
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id,
            packet: response.get(),
        }]
    }

    pub fn plan_all(
        outcomes: &[NavigatorSearchOutcome],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, connection_id))
            .collect()
    }
}
