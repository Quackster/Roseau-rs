use crate::game::catalogue::CataloguePurchaseOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CataloguePurchaseNetworkPlan;

impl CataloguePurchaseNetworkPlan {
    pub fn plan(
        outcome: &CataloguePurchaseOutcome,
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        if let Some(packet) = outcome.purchase_add_strip_item() {
            return vec![Self::write(connection_id, packet.compose().get())];
        }

        outcome
            .system_broadcast()
            .map(|packet| vec![Self::write(connection_id, packet.compose().get())])
            .unwrap_or_default()
    }

    pub fn plan_all(
        outcomes: &[CataloguePurchaseOutcome],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
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
