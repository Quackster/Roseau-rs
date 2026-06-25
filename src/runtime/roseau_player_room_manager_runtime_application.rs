use crate::game::player::{PlayerEffect, PlayerEffectRoomManagerExecutor};
use crate::game::room::RoomSummary;
use crate::runtime::RoseauApplicationRuntime;

impl RoseauApplicationRuntime {
    pub fn apply_player_room_manager_effects(
        &mut self,
        effects: &[PlayerEffect],
    ) -> Vec<RoomSummary> {
        PlayerEffectRoomManagerExecutor::apply_all(self.game_mut().room_manager_mut(), effects)
    }
}
