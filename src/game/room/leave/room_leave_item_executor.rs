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
