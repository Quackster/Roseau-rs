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
        manager.insert(PlayerSession::new(80, 30000, details(8, "bob")));
        manager
    }

    #[test]
    fn broadcasts_jump_performance_to_room_players() {
        let effects = RoomPoolNetworkPlan::plan(
            &IncomingExecutionEffect::JumpPerformance {
                data: "jump=1".to_owned(),
            },
            "alice",
            &[7, 8],
            &player_manager(),
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#JUMPDATA\ralice\rjump=1##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 80,
                    packet: "#JUMPDATA\ralice\rjump=1##".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn broadcasts_splash_position_to_room_players() {
        let effects = RoomPoolNetworkPlan::plan(
            &IncomingExecutionEffect::SplashPosition {
                position: "10,11,0.0".to_owned(),
            },
            "alice",
            &[7],
            &player_manager(),
        );

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#SHOWPROGRAM\rBIGSPLASH POSITION 10,11,0.0##".to_owned(),
            }]
        );
    }

    #[test]
    fn close_pool_change_booth_has_no_direct_network_packet() {
        assert!(RoomPoolNetworkPlan::plan(
            &IncomingExecutionEffect::ClosePoolChangeBooth,
            "alice",
            &[7],
            &player_manager(),
        )
        .is_empty());
    }
}
