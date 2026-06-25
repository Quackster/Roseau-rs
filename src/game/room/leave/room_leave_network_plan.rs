use crate::game::player::{PlayerManager, PlayerSession};
use crate::game::room::RoomLeaveEffect;
use crate::messages::outgoing::Logout;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomLeaveNetworkPlan;

impl RoomLeaveNetworkPlan {
    pub fn plan(
        effect: &RoomLeaveEffect,
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
        private_server_port: i32,
    ) -> Vec<PlayerNetworkEffect> {
        match effect {
            RoomLeaveEffect::ClosePrivateRoomConnection { user_id } => player_manager
                .get_private_room_player(*user_id, private_server_port)
                .map(|session| {
                    vec![PlayerNetworkEffect::CloseConnection {
                        connection_id: session.connection_id(),
                    }]
                })
                .unwrap_or_default(),
            RoomLeaveEffect::BroadcastLogout { username } => {
                let packet = Logout::new(username).compose().get();
                room_player_ids
                    .iter()
                    .filter_map(|user_id| player_manager.get_by_id(*user_id))
                    .map(|session| Self::write(session, packet.clone()))
                    .collect()
            }
            RoomLeaveEffect::RemovePlayerEntity { .. }
            | RoomLeaveEffect::OpenAndUnlockCurrentItem { .. }
            | RoomLeaveEffect::DisposeRoomUser { .. }
            | RoomLeaveEffect::DisposeRoomIfEmpty { .. }
            | RoomLeaveEffect::DisposeInventory { .. }
            | RoomLeaveEffect::RefreshMainMessengerStatus { .. } => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[RoomLeaveEffect],
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
        private_server_port: i32,
    ) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| {
                Self::plan(effect, room_player_ids, player_manager, private_server_port)
            })
            .collect()
    }

    fn write(session: &PlayerSession, packet: String) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id: session.connection_id(),
            packet,
        }
    }
}

#[cfg(test)]
#[path = "room_leave_network_plan_tests.rs"]
mod tests;
