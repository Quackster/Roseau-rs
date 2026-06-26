use crate::game::player::{PlayerEffect, PlayerEffectRoomLeavePlan};
use crate::game::room::RoomEffect;
use crate::runtime::RoseauApplicationRuntime;

impl RoseauApplicationRuntime {
    pub fn plan_player_room_leave_effects(&self, effects: &[PlayerEffect]) -> Vec<RoomEffect> {
        PlayerEffectRoomLeavePlan::plan_all(effects, self.game().player_manager())
    }
}
