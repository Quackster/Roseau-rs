use crate::game::room::RoomDecorationOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomDecorationNetworkPlan;

impl RoomDecorationNetworkPlan {
    pub fn plan(outcome: &RoomDecorationOutcome, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        outcome
            .flat_property_packet()
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
        outcomes: &[RoomDecorationOutcome],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, connection_id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_applied_decoration_to_current_connection_packet() {
        let effects =
            RoomDecorationNetworkPlan::plan(&RoomDecorationOutcome::applied("floor", "wood"), 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#FLATPROPERTY\rfloor/wood##".to_owned(),
            }]
        );
    }

    #[test]
    fn ignored_decoration_has_no_network_effect() {
        assert!(RoomDecorationNetworkPlan::plan(&RoomDecorationOutcome::Ignored, 42).is_empty());
    }
}
