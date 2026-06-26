use crate::game::player::PlayerManager;
use crate::game::room::RoomDecorationOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomDecorationNetworkPlan;

impl RoomDecorationNetworkPlan {
    pub fn plan(
        outcome: &RoomDecorationOutcome,
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        outcome
            .flat_property_packet()
            .map(|packet| {
                let mut response = packet.compose();
                let packet = response.get();
                room_player_ids
                    .iter()
                    .filter_map(|user_id| player_manager.get_by_id(*user_id))
                    .map(|session| PlayerNetworkEffect::WriteResponse {
                        connection_id: session.connection_id(),
                        packet: packet.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn plan_for_connection_ids(
        outcome: &RoomDecorationOutcome,
        room_connection_ids: &[i32],
    ) -> Vec<PlayerNetworkEffect> {
        outcome
            .flat_property_packet()
            .map(|packet| {
                let mut response = packet.compose();
                let packet = response.get();
                room_connection_ids
                    .iter()
                    .map(|connection_id| PlayerNetworkEffect::WriteResponse {
                        connection_id: *connection_id,
                        packet: packet.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn plan_all(
        outcomes: &[RoomDecorationOutcome],
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, room_player_ids, player_manager))
            .collect()
    }

    pub fn plan_all_for_connection_ids(
        outcomes: &[RoomDecorationOutcome],
        room_connection_ids: &[i32],
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan_for_connection_ids(outcome, room_connection_ids))
            .collect()
    }
}

#[cfg(test)]
#[path = "room_decoration_network_plan_tests.rs"]
mod tests;
