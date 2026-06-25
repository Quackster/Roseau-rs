use crate::game::item::{interactors::ItemInteractionEffect, Item};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemInteractionEffectItemExecutor;

impl ItemInteractionEffectItemExecutor {
    pub fn apply(items: &mut [Item], effect: &ItemInteractionEffect) -> Vec<Item> {
        match effect {
            ItemInteractionEffect::SetItemCustomData {
                item_id,
                custom_data,
            } => items
                .iter_mut()
                .find(|item| item.id() == *item_id)
                .map(|item| {
                    item.set_custom_data(custom_data);
                    vec![item.clone()]
                })
                .unwrap_or_default(),
            ItemInteractionEffect::RemoveStatus { .. }
            | ItemInteractionEffect::SetStatus { .. }
            | ItemInteractionEffect::SetBodyRotation { .. }
            | ItemInteractionEffect::SetPosition { .. }
            | ItemInteractionEffect::SetCanWalk { .. }
            | ItemInteractionEffect::SetWalking { .. }
            | ItemInteractionEffect::ClearNextStep
            | ItemInteractionEffect::ForceStopWalking
            | ItemInteractionEffect::MarkNeedsUpdate
            | ItemInteractionEffect::SetGoal { .. }
            | ItemInteractionEffect::BuildPathToGoal
            | ItemInteractionEffect::TriggerCurrentItem
            | ItemInteractionEffect::WalkTo { .. }
            | ItemInteractionEffect::ShowProgram { .. }
            | ItemInteractionEffect::LockTiles { .. }
            | ItemInteractionEffect::UnlockTiles { .. }
            | ItemInteractionEffect::OpenPoolChangeBooth
            | ItemInteractionEffect::SendJumpingPlaceOk
            | ItemInteractionEffect::SendJumpData { .. }
            | ItemInteractionEffect::DecrementTickets { .. }
            | ItemInteractionEffect::SendTickets
            | ItemInteractionEffect::SavePlayer
            | ItemInteractionEffect::SendDoorOut { .. }
            | ItemInteractionEffect::SendDoorIn { .. }
            | ItemInteractionEffect::LoadRoom { .. }
            | ItemInteractionEffect::LeaveRoom { .. }
            | ItemInteractionEffect::UpdateItemStatus { .. }
            | ItemInteractionEffect::Schedule { .. } => Vec::new(),
        }
    }

    pub fn apply_all(items: &mut [Item], effects: &[ItemInteractionEffect]) -> Vec<Item> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(items, effect))
            .collect()
    }
}

#[cfg(test)]
#[path = "item_interaction_effect_item_executor_tests.rs"]
mod tests;
