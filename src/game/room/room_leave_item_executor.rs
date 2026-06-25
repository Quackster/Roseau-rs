use crate::game::item::Item;
use crate::game::room::{RoomLeaveEffect, RoomMapping};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomLeaveItemExecutor;

impl RoomLeaveItemExecutor {
    pub fn apply(
        items: &mut [Item],
        mapping: &mut RoomMapping,
        effect: &RoomLeaveEffect,
    ) -> Vec<Item> {
        match effect {
            RoomLeaveEffect::OpenAndUnlockCurrentItem { item_id } => items
                .iter_mut()
                .find(|item| item.id() == *item_id)
                .map(|item| {
                    item.set_current_program(Some("open".to_owned()));
                    mapping.set_item_tiles_override_lock(item, false);
                    vec![item.clone()]
                })
                .unwrap_or_default(),
            RoomLeaveEffect::ClosePrivateRoomConnection { .. }
            | RoomLeaveEffect::RemovePlayerEntity { .. }
            | RoomLeaveEffect::DisposeRoomUser { .. }
            | RoomLeaveEffect::BroadcastLogout { .. }
            | RoomLeaveEffect::DisposeRoomIfEmpty { .. }
            | RoomLeaveEffect::DisposeInventory { .. }
            | RoomLeaveEffect::RefreshMainMessengerStatus { .. } => Vec::new(),
        }
    }

    pub fn apply_all(
        items: &mut [Item],
        mapping: &mut RoomMapping,
        effects: &[RoomLeaveEffect],
    ) -> Vec<Item> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(items, mapping, effect))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;
    use crate::game::room::model::RoomModel;

    fn pool_item(item_id: i32, sprite: &str, custom_data: Option<String>) -> Item {
        Item::new(
            item_id,
            12,
            7,
            "1",
            1,
            0.0,
            0,
            ItemDefinition::new(item_id, sprite, "", 1, 1, 0.0, "SF", "", "", ""),
            "",
            custom_data,
        )
        .unwrap()
    }

    fn mapping(items: &[Item]) -> RoomMapping {
        let mut mapping = RoomMapping::new(
            RoomModel::new("model", "000 000 000", 0, 0, 0, 0, true, false).unwrap(),
        );
        mapping.regenerate_collision_maps(items.to_vec());
        for item in items {
            mapping.set_item_tiles_override_lock(item, true);
        }
        mapping
    }

    #[test]
    fn opens_current_item_and_unlocks_primary_and_custom_tiles() {
        let mut items = vec![pool_item(9, "poolLift", Some("2,2".to_owned()))];
        let mut mapping = mapping(&items);

        assert!(mapping.tile(1, 1).unwrap().has_override_lock());
        assert!(mapping.tile(2, 2).unwrap().has_override_lock());

        let updated = RoomLeaveItemExecutor::apply(
            &mut items,
            &mut mapping,
            &RoomLeaveEffect::OpenAndUnlockCurrentItem { item_id: 9 },
        );

        assert_eq!(updated.len(), 1);
        assert_eq!(updated[0].current_program(), Some("open"));
        assert_eq!(items[0].current_program(), Some("open"));
        assert!(!mapping.tile(1, 1).unwrap().has_override_lock());
        assert!(!mapping.tile(2, 2).unwrap().has_override_lock());
    }

    #[test]
    fn ignores_missing_items_and_other_leave_effects() {
        let mut items = vec![pool_item(9, "poolBooth", None)];
        let mut mapping = mapping(&items);

        let updated = RoomLeaveItemExecutor::apply_all(
            &mut items,
            &mut mapping,
            &[
                RoomLeaveEffect::OpenAndUnlockCurrentItem { item_id: 99 },
                RoomLeaveEffect::BroadcastLogout {
                    username: "alice".to_owned(),
                },
            ],
        );

        assert!(updated.is_empty());
        assert_eq!(items[0].current_program(), None);
        assert!(mapping.tile(1, 1).unwrap().has_override_lock());
    }
}
