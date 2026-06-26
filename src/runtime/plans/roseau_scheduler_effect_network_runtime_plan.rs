use crate::game::room::entity::RoomUser;
use crate::game::room::schedulers::{SchedulerEffect, SchedulerEffectNetworkPlan};
use crate::runtime::RoseauApplicationRuntime;
use crate::server::PlayerNetworkEffect;

impl RoseauApplicationRuntime {
    pub fn plan_scheduler_effect_network_effects(
        &self,
        effects: &[SchedulerEffect],
        room_player_ids: &[i32],
        room_users: &[RoomUser],
    ) -> Vec<PlayerNetworkEffect> {
        SchedulerEffectNetworkPlan::plan_all(
            effects,
            room_player_ids,
            room_users,
            self.game().player_manager(),
        )
    }

    pub fn apply_scheduler_effect_network_effects(
        &mut self,
        effects: &[SchedulerEffect],
        room_player_ids: &[i32],
        room_users: &[RoomUser],
    ) -> Vec<PlayerNetworkEffect> {
        let network_effects =
            self.plan_scheduler_effect_network_effects(effects, room_player_ids, room_users);

        self.startup_runtime_mut()
            .apply_network_effects(network_effects)
    }
}
