use crate::game::{RoomEffect, RoomEffectManagerExecutor};
use crate::runtime::RoseauApplicationRuntime;

impl RoseauApplicationRuntime {
    pub fn apply_room_manager_effects(&mut self, effects: &[RoomEffect]) -> usize {
        RoomEffectManagerExecutor::apply_all(self.game_mut().room_manager_mut(), effects)
    }
}
