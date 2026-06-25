use crate::game::item::interactors::ItemInteractionRuntimeEffect;
use crate::game::player::PlayerManager;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemInteractionRuntimeExecutor;

impl ItemInteractionRuntimeExecutor {
    pub fn apply(
        player_manager: &mut PlayerManager,
        effect: &ItemInteractionRuntimeEffect,
    ) -> Vec<ItemInteractionRuntimeEffect> {
        match effect {
            ItemInteractionRuntimeEffect::SyncPlayerTickets { user_id, tickets } => {
                player_manager.sync_player_tickets(*user_id, *tickets);
                Vec::new()
            }
            ItemInteractionRuntimeEffect::ScheduleEffects { .. }
            | ItemInteractionRuntimeEffect::LoadRoom { .. }
            | ItemInteractionRuntimeEffect::LeaveRoom { .. } => vec![effect.clone()],
        }
    }

    pub fn apply_all(
        player_manager: &mut PlayerManager,
        effects: &[ItemInteractionRuntimeEffect],
    ) -> Vec<ItemInteractionRuntimeEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(player_manager, effect))
            .collect()
    }
}
