use crate::game::{RoomEffect, RoomEffectNetworkPlan};
use crate::runtime::RoseauApplicationRuntime;
use crate::server::PlayerNetworkEffect;

impl RoseauApplicationRuntime {
    pub fn plan_room_effect_network_effects(
        &self,
        effects: &[RoomEffect],
    ) -> Vec<PlayerNetworkEffect> {
        RoomEffectNetworkPlan::plan_all(effects, self.game().player_manager())
    }

    pub fn apply_room_effect_network_effects(
        &mut self,
        effects: &[RoomEffect],
    ) -> Vec<PlayerNetworkEffect> {
        let network_effects = self.plan_room_effect_network_effects(effects);

        self.startup_runtime_mut()
            .apply_network_effects(network_effects)
    }
}
