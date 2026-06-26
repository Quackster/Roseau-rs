use crate::game::player::PlayerManager;
use crate::game::room::{Room, RoomEffect, RoomLeaveEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomLeaveRoomExecutor;

impl RoomLeaveRoomExecutor {
    pub fn apply(
        room: &mut Room,
        player_manager: &PlayerManager,
        effect: &RoomLeaveEffect,
    ) -> Vec<RoomEffect> {
        match effect {
            RoomLeaveEffect::RemovePlayerEntity { user_id } => {
                room.remove_player(*user_id);
                Vec::new()
            }
            RoomLeaveEffect::DisposeRoomIfEmpty { room_id } if *room_id == room.data().id() => {
                room.dispose(false, player_manager)
            }
            _ => Vec::new(),
        }
    }

    pub fn apply_all(
        room: &mut Room,
        player_manager: &PlayerManager,
        effects: &[RoomLeaveEffect],
    ) -> Vec<RoomEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(room, player_manager, effect))
            .collect()
    }
}

#[cfg(test)]
#[path = "room_leave_room_executor_tests.rs"]
mod tests;
