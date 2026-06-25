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
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;

    fn item(id: i32, custom_data: Option<String>) -> Item {
        Item::new(
            id,
            1,
            7,
            "1",
            2,
            0.0,
            0,
            ItemDefinition::new(id, "teleport", "", 1, 1, 0.0, "SFX", "", "", "DOOROPEN"),
            "",
            custom_data,
        )
        .unwrap()
    }

    #[test]
    fn applies_runtime_custom_data_to_matching_item() {
        let mut items = vec![item(7, Some("FALSE".to_owned())), item(8, None)];

        let updated = ItemInteractionEffectItemExecutor::apply(
            &mut items,
            &ItemInteractionEffect::SetItemCustomData {
                item_id: 7,
                custom_data: "TRUE".to_owned(),
            },
        );

        assert_eq!(items[0].custom_data(), Some("TRUE"));
        assert_eq!(updated.len(), 1);
        assert_eq!(updated[0].id(), 7);
        assert_eq!(updated[0].custom_data(), Some("TRUE"));
        assert_eq!(items[1].custom_data(), None);
    }

    #[test]
    fn ignores_missing_items_and_other_boundaries() {
        let mut items = vec![item(7, Some("FALSE".to_owned()))];

        let updated = ItemInteractionEffectItemExecutor::apply_all(
            &mut items,
            &[
                ItemInteractionEffect::SetItemCustomData {
                    item_id: 99,
                    custom_data: "TRUE".to_owned(),
                },
                ItemInteractionEffect::UpdateItemStatus { item_id: 7 },
                ItemInteractionEffect::SavePlayer,
            ],
        );

        assert!(updated.is_empty());
        assert_eq!(items[0].custom_data(), Some("FALSE"));
    }
}
