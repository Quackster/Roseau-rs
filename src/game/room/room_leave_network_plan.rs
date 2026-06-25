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
mod tests {
    use super::*;
    use crate::game::player::{PlayerDetails, PlayerSession};

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    fn player_manager() -> PlayerManager {
        let mut manager = PlayerManager::new(Vec::new());
        manager.insert(PlayerSession::new(70, 30000, details(7, "alice")));
        manager.insert(PlayerSession::new(71, 37119, details(7, "alice")));
        manager.insert(PlayerSession::new(80, 30000, details(8, "bob")));
        manager
    }

    #[test]
    fn closes_private_room_connection_on_matching_private_port() {
        let effects = RoomLeaveNetworkPlan::plan(
            &RoomLeaveEffect::ClosePrivateRoomConnection { user_id: 7 },
            &[7, 8],
            &player_manager(),
            37119,
        );

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::CloseConnection { connection_id: 71 }]
        );
    }

    #[test]
    fn broadcasts_logout_to_current_room_players() {
        let effects = RoomLeaveNetworkPlan::plan(
            &RoomLeaveEffect::BroadcastLogout {
                username: "alice".to_owned(),
            },
            &[8],
            &player_manager(),
            37119,
        );

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#LOGOUT\ralice##".to_owned(),
            }]
        );
    }

    #[test]
    fn plans_room_leave_network_effects_in_order() {
        let effects = RoomLeaveNetworkPlan::plan_all(
            &[
                RoomLeaveEffect::ClosePrivateRoomConnection { user_id: 7 },
                RoomLeaveEffect::RemovePlayerEntity { user_id: 7 },
                RoomLeaveEffect::BroadcastLogout {
                    username: "alice".to_owned(),
                },
            ],
            &[8],
            &player_manager(),
            37119,
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::CloseConnection { connection_id: 71 },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 80,
                    packet: "#LOGOUT\ralice##".to_owned(),
                },
            ]
        );
    }
}
