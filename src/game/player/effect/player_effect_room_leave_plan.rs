use crate::game::player::{PlayerEffect, PlayerManager};
use crate::game::room::RoomEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerEffectRoomLeavePlan;

impl PlayerEffectRoomLeavePlan {
    pub fn plan(effect: &PlayerEffect, player_manager: &PlayerManager) -> Vec<RoomEffect> {
        match effect {
            PlayerEffect::LeaveCurrentRoom { connection_id } => player_manager
                .players()
                .get(connection_id)
                .map(|session| {
                    vec![RoomEffect::LeaveRoom {
                        user_id: session.details().id(),
                        hotel_view: false,
                    }]
                })
                .unwrap_or_default(),
            PlayerEffect::SendAlert(_)
            | PlayerEffect::UpdateLastLogin { .. }
            | PlayerEffect::CloseConnection { .. }
            | PlayerEffect::CloseUserConnections { .. }
            | PlayerEffect::DisposeOwnedRooms { .. }
            | PlayerEffect::DisposeInventory { .. }
            | PlayerEffect::Messenger(_) => Vec::new(),
        }
    }

    pub fn plan_all(effects: &[PlayerEffect], player_manager: &PlayerManager) -> Vec<RoomEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, player_manager))
            .collect()
    }
}

#[cfg(test)]
#[path = "player_effect_room_leave_plan_tests.rs"]
mod tests;
