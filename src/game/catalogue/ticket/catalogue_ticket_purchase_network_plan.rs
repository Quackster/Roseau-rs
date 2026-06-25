use crate::game::catalogue::CatalogueTicketPurchaseOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CatalogueTicketPurchaseNetworkPlan;

impl CatalogueTicketPurchaseNetworkPlan {
    pub fn plan(
        outcome: &CatalogueTicketPurchaseOutcome,
        buyer_connection_id: i32,
        target_connection_id: Option<i32>,
    ) -> Vec<PlayerNetworkEffect> {
        let mut effects = vec![Self::write(
            buyer_connection_id,
            outcome.buyer_alert().compose().get(),
        )];

        if let (Some(packet), Some(connection_id)) = (outcome.target_alert(), target_connection_id)
        {
            effects.push(Self::write(connection_id, packet.compose().get()));
        }

        effects
    }

    pub fn plan_all(
        outcomes: &[CatalogueTicketPurchaseOutcome],
        buyer_connection_id: i32,
        target_connection_id: Option<i32>,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, buyer_connection_id, target_connection_id))
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
#[path = "catalogue_ticket_purchase_network_plan_tests.rs"]
mod tests;
