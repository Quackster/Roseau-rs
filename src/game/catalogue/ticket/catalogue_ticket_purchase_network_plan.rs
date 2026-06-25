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
mod tests {
    use super::*;

    #[test]
    fn maps_self_purchase_to_buyer_alert() {
        let effects = CatalogueTicketPurchaseNetworkPlan::plan(
            &CatalogueTicketPurchaseOutcome::BoughtForSelf,
            42,
            None,
        );

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#SYSTEMBROADCAST\rYou have bought 10 game tickets!##".to_owned(),
            }]
        );
    }

    #[test]
    fn maps_cross_player_purchase_to_buyer_and_target_alerts() {
        let effects = CatalogueTicketPurchaseNetworkPlan::plan(
            &CatalogueTicketPurchaseOutcome::BoughtForOther {
                buyer_username: "alice".to_owned(),
                target_username: "bob".to_owned(),
            },
            42,
            Some(77),
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 42,
                    packet: "#SYSTEMBROADCAST\rYou have bought 10 game tickets for bob##"
                        .to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 77,
                    packet: "#SYSTEMBROADCAST\ralice has bought 10 game tickets for you!##"
                        .to_owned(),
                },
            ]
        );
    }

    #[test]
    fn omits_target_alert_when_target_connection_is_missing() {
        let effects = CatalogueTicketPurchaseNetworkPlan::plan(
            &CatalogueTicketPurchaseOutcome::BoughtForOther {
                buyer_username: "alice".to_owned(),
                target_username: "bob".to_owned(),
            },
            42,
            None,
        );

        assert_eq!(effects.len(), 1);
    }
}
