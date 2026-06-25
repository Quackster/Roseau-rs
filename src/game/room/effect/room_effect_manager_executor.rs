use crate::game::room::{RoomEffect, RoomManager};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEffectManagerExecutor;

impl RoomEffectManagerExecutor {
    pub fn apply(room_manager: &mut RoomManager, effect: &RoomEffect) -> bool {
        match effect {
            RoomEffect::RemoveLoadedRoom { room_id } => {
                room_manager.remove_loaded_room(*room_id).is_some()
            }
            _ => false,
        }
    }

    pub fn apply_all(room_manager: &mut RoomManager, effects: &[RoomEffect]) -> usize {
        effects
            .iter()
            .filter(|effect| Self::apply(room_manager, effect))
            .count()
    }
}
