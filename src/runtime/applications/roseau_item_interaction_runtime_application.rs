use crate::game::item::{ItemInteractionRuntimeEffect, ItemInteractionRuntimeExecutor};
use crate::runtime::RoseauApplicationRuntime;

impl RoseauApplicationRuntime {
    pub fn apply_item_interaction_runtime_effects(
        &mut self,
        effects: &[ItemInteractionRuntimeEffect],
    ) -> Vec<ItemInteractionRuntimeEffect> {
        ItemInteractionRuntimeExecutor::apply_all(self.game_mut().player_manager_mut(), effects)
    }
}
