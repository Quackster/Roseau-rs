use crate::game::messenger::MessengerEffectNetworkPlan;
use crate::game::player::{PlayerEffect, PlayerManager};
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerEffectNetworkPlan;

impl PlayerEffectNetworkPlan {
    pub fn plan(effect: &PlayerEffect, player_manager: &PlayerManager) -> Vec<PlayerNetworkEffect> {
        match effect {
            PlayerEffect::SendAlert(packet) => player_manager
                .players()
                .values()
                .map(|session| PlayerNetworkEffect::WriteResponse {
                    connection_id: session.connection_id(),
                    packet: packet.clone().compose().get(),
                })
                .collect(),
            PlayerEffect::CloseConnection { connection_id } => {
                vec![PlayerNetworkEffect::CloseConnection {
                    connection_id: *connection_id,
                }]
            }
            PlayerEffect::CloseUserConnections { user_id } => player_manager
                .players()
                .values()
                .filter(|session| session.details().id() == *user_id)
                .map(|session| PlayerNetworkEffect::CloseConnection {
                    connection_id: session.connection_id(),
                })
                .collect(),
            PlayerEffect::Messenger(messenger_effect) => player_manager
                .get_by_id(messenger_effect.user_id_hint())
                .map(|session| {
                    MessengerEffectNetworkPlan::plan(messenger_effect, session.connection_id())
                })
                .unwrap_or_default(),
            PlayerEffect::UpdateLastLogin { .. }
            | PlayerEffect::DisposeOwnedRooms { .. }
            | PlayerEffect::DisposeInventory { .. }
            | PlayerEffect::LeaveCurrentRoom { .. } => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[PlayerEffect],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, player_manager))
            .collect()
    }
}

trait MessengerEffectUserHint {
    fn user_id_hint(&self) -> i32;
}

impl MessengerEffectUserHint for crate::game::messenger::MessengerEffect {
    fn user_id_hint(&self) -> i32 {
        match self {
            Self::RefreshFriendList { user_id, .. } => *user_id,
            Self::SendRequests(_) | Self::SendFriends(_) => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::player::{PlayerDetails, PlayerSession};
    use crate::messages::outgoing::SystemBroadcast;

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    fn player_manager() -> PlayerManager {
        let mut manager = PlayerManager::new(Vec::new());
        manager.insert(PlayerSession::new(10, 30000, details(1, "alice")));
        manager.insert(PlayerSession::new(11, 30001, details(1, "alice-room")));
        manager.insert(PlayerSession::new(20, 30000, details(2, "bob")));
        manager
    }

    #[test]
    fn plans_alerts_and_direct_closes() {
        let manager = player_manager();

        let effects = PlayerEffectNetworkPlan::plan_all(
            &[
                PlayerEffect::SendAlert(SystemBroadcast::new("maintenance")),
                PlayerEffect::CloseConnection { connection_id: 20 },
            ],
            &manager,
        );

        assert_eq!(effects.len(), 4);
        assert!(effects.contains(&PlayerNetworkEffect::WriteResponse {
            connection_id: 10,
            packet: "#SYSTEMBROADCAST\rmaintenance##".to_owned(),
        }));
        assert!(effects.contains(&PlayerNetworkEffect::WriteResponse {
            connection_id: 11,
            packet: "#SYSTEMBROADCAST\rmaintenance##".to_owned(),
        }));
        assert!(effects.contains(&PlayerNetworkEffect::WriteResponse {
            connection_id: 20,
            packet: "#SYSTEMBROADCAST\rmaintenance##".to_owned(),
        }));
        assert!(effects.contains(&PlayerNetworkEffect::CloseConnection { connection_id: 20 }));
    }

    #[test]
    fn closes_all_connections_for_user() {
        let manager = player_manager();

        let effects = PlayerEffectNetworkPlan::plan(
            &PlayerEffect::CloseUserConnections { user_id: 1 },
            &manager,
        );

        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&PlayerNetworkEffect::CloseConnection { connection_id: 10 }));
        assert!(effects.contains(&PlayerNetworkEffect::CloseConnection { connection_id: 11 }));
    }

    #[test]
    fn ignores_persistence_and_cleanup_effects() {
        let manager = player_manager();

        assert!(PlayerEffectNetworkPlan::plan(
            &PlayerEffect::UpdateLastLogin { user_id: 1 },
            &manager,
        )
        .is_empty());
        assert!(PlayerEffectNetworkPlan::plan(
            &PlayerEffect::DisposeInventory { user_id: 1 },
            &manager,
        )
        .is_empty());
        assert!(PlayerEffectNetworkPlan::plan(
            &PlayerEffect::LeaveCurrentRoom { connection_id: 10 },
            &manager,
        )
        .is_empty());
    }
}
