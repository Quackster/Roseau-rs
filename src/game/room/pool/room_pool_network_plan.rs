use crate::game::player::PlayerManager;
use crate::messages::outgoing::JumpData;
use crate::messages::{IncomingExecutionEffect, OutgoingMessage};
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomPoolNetworkPlan;

impl RoomPoolNetworkPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        username: &str,
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        match effect {
            IncomingExecutionEffect::JumpPerformance { data } => Self::broadcast(
                room_player_ids,
                player_manager,
                JumpData::new(username, data).compose().get(),
            ),
            IncomingExecutionEffect::SplashPosition { .. } => Vec::new(),
            IncomingExecutionEffect::ClosePoolChangeBooth => Vec::new(),
            _ => Vec::new(),
        }
    }

    pub fn plan_for_connection_ids(
        effect: &IncomingExecutionEffect,
        username: &str,
        room_connection_ids: &[i32],
    ) -> Vec<PlayerNetworkEffect> {
        match effect {
            IncomingExecutionEffect::JumpPerformance { data } => Self::broadcast_to_connection_ids(
                room_connection_ids,
                JumpData::new(username, data).compose().get(),
            ),
            IncomingExecutionEffect::SplashPosition { .. } => Vec::new(),
            IncomingExecutionEffect::ClosePoolChangeBooth => Vec::new(),
            _ => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        username: &str,
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, username, room_player_ids, player_manager))
            .collect()
    }

    pub fn plan_all_for_connection_ids(
        effects: &[IncomingExecutionEffect],
        username: &str,
        room_connection_ids: &[i32],
    ) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan_for_connection_ids(effect, username, room_connection_ids))
            .collect()
    }

    fn broadcast(
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
        packet: String,
    ) -> Vec<PlayerNetworkEffect> {
        room_player_ids
            .iter()
            .filter_map(|user_id| player_manager.get_by_id(*user_id))
            .map(|session| PlayerNetworkEffect::WriteResponse {
                connection_id: session.connection_id(),
                packet: packet.clone(),
            })
            .collect()
    }

    fn broadcast_to_connection_ids(
        room_connection_ids: &[i32],
        packet: String,
    ) -> Vec<PlayerNetworkEffect> {
        room_connection_ids
            .iter()
            .map(|connection_id| PlayerNetworkEffect::WriteResponse {
                connection_id: *connection_id,
                packet: packet.clone(),
            })
            .collect()
    }
}

#[cfg(test)]
#[path = "room_pool_network_plan_tests.rs"]
mod tests;
