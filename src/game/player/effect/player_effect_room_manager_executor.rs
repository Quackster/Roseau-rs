use crate::game::player::PlayerEffect;
use crate::game::room::{RoomManager, RoomSummary};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerEffectRoomManagerExecutor;

impl PlayerEffectRoomManagerExecutor {
    pub fn apply(room_manager: &mut RoomManager, effect: &PlayerEffect) -> Vec<RoomSummary> {
        match effect {
            PlayerEffect::DisposeOwnedRooms { user_id } => {
                let room_ids = room_manager
                    .get_player_rooms(*user_id)
                    .into_iter()
                    .map(|room| room.data().id())
                    .collect::<Vec<_>>();

                room_ids
                    .into_iter()
                    .filter_map(|room_id| room_manager.remove_loaded_room(room_id))
                    .collect()
            }
            PlayerEffect::SendAlert(_)
            | PlayerEffect::UpdateLastLogin { .. }
            | PlayerEffect::CloseConnection { .. }
            | PlayerEffect::CloseUserConnections { .. }
            | PlayerEffect::DisposeInventory { .. }
            | PlayerEffect::LeaveCurrentRoom { .. }
            | PlayerEffect::Messenger(_) => Vec::new(),
        }
    }

    pub fn apply_all(room_manager: &mut RoomManager, effects: &[PlayerEffect]) -> Vec<RoomSummary> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(room_manager, effect))
            .collect()
    }
}

#[cfg(test)]
#[path = "player_effect_room_manager_executor_tests.rs"]
mod tests;
