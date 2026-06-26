use crate::game::{ModerationEffect, ModerationEffectNetworkPlan};
use crate::runtime::RoseauApplicationRuntime;
use crate::server::PlayerNetworkEffect;

impl RoseauApplicationRuntime {
    pub fn plan_moderation_effect_network_effects(
        &self,
        effects: &[ModerationEffect],
    ) -> Vec<PlayerNetworkEffect> {
        ModerationEffectNetworkPlan::plan_all(effects)
    }

    pub fn apply_moderation_effect_network_effects(
        &mut self,
        effects: &[ModerationEffect],
    ) -> Vec<PlayerNetworkEffect> {
        let network_effects = self.plan_moderation_effect_network_effects(effects);

        self.startup_runtime_mut()
            .apply_network_effects(network_effects)
    }
}
