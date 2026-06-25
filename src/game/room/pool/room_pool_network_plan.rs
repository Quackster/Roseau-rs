use crate::game::player::PlayerManager;
use crate::messages::outgoing::{JumpData, ShowProgram};
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
            IncomingExecutionEffect::SplashPosition { position } => Self::broadcast(
                room_player_ids,
                player_manager,
                ShowProgram::new(["BIGSPLASH", "POSITION", position])
                    .compose()
                    .get(),
            ),
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
}

#[cfg(test)]
#[path = "room_pool_network_plan_tests.rs"]
mod tests;
