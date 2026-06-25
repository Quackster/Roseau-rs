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
