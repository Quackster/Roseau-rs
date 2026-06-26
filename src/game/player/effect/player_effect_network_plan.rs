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
#[path = "player_effect_network_plan_tests.rs"]
mod tests;
