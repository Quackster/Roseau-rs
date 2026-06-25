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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_successful_purchase_to_add_strip_item_packet() {
        let effects =
            CataloguePurchaseNetworkPlan::plan(&CataloguePurchaseOutcome::AddedStripItem, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#ADDSTRIPITEM##".to_owned(),
            }]
        );
    }

    #[test]
    fn maps_insufficient_credits_to_system_broadcast_packet() {
        let effects =
            CataloguePurchaseNetworkPlan::plan(&CataloguePurchaseOutcome::NotEnoughCredits, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#SYSTEMBROADCAST\rYou don't have enough credits to purchase this item!##"
                    .to_owned(),
            }]
        );
    }
}
