use crate::game::room::{RoomLeaveEffect, RoomLeaveNetworkPlan};
use crate::runtime::RoseauApplicationRuntime;
use crate::server::PlayerNetworkEffect;

impl RoseauApplicationRuntime {
    pub fn plan_room_leave_network_effects(
        &self,
        effects: &[RoomLeaveEffect],
        room_player_ids: &[i32],
        private_server_port: i32,
    ) -> Vec<PlayerNetworkEffect> {
        RoomLeaveNetworkPlan::plan_all(
            effects,
            room_player_ids,
            self.game().player_manager(),
            private_server_port,
        )
    }

    pub fn apply_room_leave_network_effects(
        &mut self,
        effects: &[RoomLeaveEffect],
        room_player_ids: &[i32],
        private_server_port: i32,
    ) -> Vec<PlayerNetworkEffect> {
        let network_effects =
            self.plan_room_leave_network_effects(effects, room_player_ids, private_server_port);

        self.startup_runtime_mut()
            .apply_network_effects(network_effects)
    }
}
