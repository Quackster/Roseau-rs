use crate::game::room::entity::{RoomUser, RoomUserEffect, RoomUserEffectNetworkPlan};
use crate::runtime::RoseauApplicationRuntime;
use crate::server::PlayerNetworkEffect;

impl RoseauApplicationRuntime {
    pub fn plan_room_user_effect_network_effects(
        &self,
        effects: &[RoomUserEffect],
        acting_user_id: i32,
        room_player_ids: &[i32],
        room_users: &[RoomUser],
    ) -> Vec<PlayerNetworkEffect> {
        RoomUserEffectNetworkPlan::plan_all(
            effects,
            acting_user_id,
            room_player_ids,
            room_users,
            self.game().player_manager(),
        )
    }

    pub fn apply_room_user_effect_network_effects(
        &mut self,
        effects: &[RoomUserEffect],
        acting_user_id: i32,
        room_player_ids: &[i32],
        room_users: &[RoomUser],
    ) -> Vec<PlayerNetworkEffect> {
        let network_effects = self.plan_room_user_effect_network_effects(
            effects,
            acting_user_id,
            room_player_ids,
            room_users,
        );

        self.startup_runtime_mut()
            .apply_network_effects(network_effects)
    }
}
