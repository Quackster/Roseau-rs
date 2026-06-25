use std::collections::VecDeque;

use crate::game::item::{interactors::ItemInteractionEffect, Item};
use crate::game::pathfinder::make_path;
use crate::game::room::entity::{RoomUser, RoomUserEffect};
use crate::game::room::{RoomMapping, RoomOccupant};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemInteractionEffectRoomExecutor;

impl ItemInteractionEffectRoomExecutor {
    pub fn apply(
        user: &mut RoomUser,
        effect: &ItemInteractionEffect,
        mapping: &mut RoomMapping,
        items: &[Item],
        occupants: &[RoomOccupant],
        pool_figure_available: bool,
        acting_tickets: i32,
    ) -> Vec<RoomUserEffect> {
        match effect {
            ItemInteractionEffect::LockTiles { item_id } => {
                Self::set_item_tiles_override_lock(mapping, items, *item_id, true);
                Vec::new()
            }
            ItemInteractionEffect::UnlockTiles { item_id } => {
                Self::set_item_tiles_override_lock(mapping, items, *item_id, false);
                Vec::new()
            }
            ItemInteractionEffect::WalkTo { x, y } => Self::walk_to(
                user,
                *x,
                *y,
                mapping,
                items,
                occupants,
                pool_figure_available,
                acting_tickets,
            ),
            ItemInteractionEffect::BuildPathToGoal => user
                .goal()
                .map(|goal| {
                    Self::walk_to(
                        user,
                        goal.x(),
                        goal.y(),
                        mapping,
                        items,
                        occupants,
                        pool_figure_available,
                        acting_tickets,
                    )
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
            | ItemInteractionEffect::TriggerCurrentItem
            | ItemInteractionEffect::ShowProgram { .. }
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
            | ItemInteractionEffect::SetItemCustomData { .. }
            | ItemInteractionEffect::UpdateItemStatus { .. }
            | ItemInteractionEffect::Schedule { .. } => Vec::new(),
        }
    }

    pub fn apply_all(
        user: &mut RoomUser,
        effects: &[ItemInteractionEffect],
        mapping: &mut RoomMapping,
        items: &[Item],
        occupants: &[RoomOccupant],
        pool_figure_available: bool,
        acting_tickets: i32,
    ) -> Vec<RoomUserEffect> {
        effects
            .iter()
            .flat_map(|effect| {
                Self::apply(
                    user,
                    effect,
                    mapping,
                    items,
                    occupants,
                    pool_figure_available,
                    acting_tickets,
                )
            })
            .collect()
    }

    fn set_item_tiles_override_lock(
        mapping: &mut RoomMapping,
        items: &[Item],
        item_id: i32,
        override_lock: bool,
    ) {
        if let Some(item) = items.iter().find(|item| item.id() == item_id) {
            mapping.set_item_tiles_override_lock(item, override_lock);
        }
    }

    fn walk_to(
        user: &mut RoomUser,
        x: i32,
        y: i32,
        mapping: &RoomMapping,
        items: &[Item],
        occupants: &[RoomOccupant],
        pool_figure_available: bool,
        acting_tickets: i32,
    ) -> Vec<RoomUserEffect> {
        if !mapping.is_valid_tile(
            user.entity_id(),
            x,
            y,
            items,
            occupants,
            pool_figure_available,
        ) {
            return Vec::new();
        }

        if Self::needs_pool_ticket(mapping, items, x, y) && acting_tickets <= 0 {
            return vec![RoomUserEffect::NotEnoughTickets];
        }

        let path = make_path(
            user.position(),
            crate::game::room::model::Position::new(x, y, 0.0),
            mapping.model().map_size_x(),
            mapping.model().map_size_y(),
            |_current, next, _final_move| {
                mapping.is_valid_tile(
                    user.entity_id(),
                    next.x(),
                    next.y(),
                    items,
                    occupants,
                    pool_figure_available,
                )
            },
        );

        user.walk_to(x, y, VecDeque::from(path));
        Vec::new()
    }

    fn needs_pool_ticket(mapping: &RoomMapping, items: &[Item], x: i32, y: i32) -> bool {
        mapping
            .highest_item_id(x, y)
            .and_then(|item_id| items.iter().find(|item| item.id() == item_id))
            .is_some_and(|item| matches!(item.definition().sprite(), "poolLift" | "poolQueue"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;
    use crate::game::room::model::{Position, RoomModel};

    fn model() -> RoomModel {
        RoomModel::new("room", "0000 0000 0000 0000", 0, 0, 0, 0, false, false).unwrap()
    }

    fn item(id: i32, sprite: &str, x: &str, y: i32, custom_data: Option<String>) -> Item {
        Item::new(
            id,
            1,
            1,
            x,
            y,
            0.0,
            0,
            ItemDefinition::new(id, sprite, "", 1, 1, 0.0, "SF", "", "", ""),
            "",
            custom_data,
        )
        .unwrap()
    }

    fn user() -> RoomUser {
        let mut user = RoomUser::new(7, "Alice", "hd-100", "hello", Some("pool"));
        user.set_room_id(1);
        user.set_position(Position::new(0, 0, 0.0));
        user
    }

    #[test]
    fn locks_and_unlocks_item_tiles() {
        let mut mapping = RoomMapping::new(model());
        let lock = item(10, "poolLift", "1", 1, Some("2,2".to_owned()));
        let items = vec![lock.clone()];
        mapping.regenerate_collision_maps(items.clone());
        let mut user = user();

        ItemInteractionEffectRoomExecutor::apply(
            &mut user,
            &ItemInteractionEffect::LockTiles { item_id: 10 },
            &mut mapping,
            &items,
            &[],
            true,
            1,
        );

        assert!(mapping.tile(1, 1).unwrap().has_override_lock());
        assert!(mapping.tile(2, 2).unwrap().has_override_lock());

        ItemInteractionEffectRoomExecutor::apply(
            &mut user,
            &ItemInteractionEffect::UnlockTiles { item_id: 10 },
            &mut mapping,
            &items,
            &[],
            true,
            1,
        );

        assert!(!mapping.tile(1, 1).unwrap().has_override_lock());
        assert!(!mapping.tile(2, 2).unwrap().has_override_lock());
    }

    #[test]
    fn walks_to_valid_target_with_built_path() {
        let mut mapping = RoomMapping::new(model());
        let mut user = user();

        let effects = ItemInteractionEffectRoomExecutor::apply(
            &mut user,
            &ItemInteractionEffect::WalkTo { x: 3, y: 0 },
            &mut mapping,
            &[],
            &[],
            false,
            0,
        );

        assert!(effects.is_empty());
        assert!(user.is_walking());
        assert_eq!(user.goal(), Some(Position::new(3, 0, 0.0)));
        assert_eq!(user.path().back().copied(), Some(Position::new(3, 0, 0.0)));
    }

    #[test]
    fn builds_path_to_existing_goal_for_pool_ladder_effect() {
        let mut mapping = RoomMapping::new(model());
        let mut user = user();
        user.set_goal(Some(Position::new(0, 3, 0.0)));

        ItemInteractionEffectRoomExecutor::apply(
            &mut user,
            &ItemInteractionEffect::BuildPathToGoal,
            &mut mapping,
            &[],
            &[],
            true,
            0,
        );

        assert!(user.is_walking());
        assert_eq!(user.goal(), Some(Position::new(0, 3, 0.0)));
        assert_eq!(user.path().back().copied(), Some(Position::new(0, 3, 0.0)));
    }

    #[test]
    fn sends_no_ticket_effect_for_pool_targets() {
        let mut mapping = RoomMapping::new(model());
        let lift = item(11, "poolLift", "1", 0, None);
        let items = vec![lift];
        mapping.regenerate_collision_maps(items.clone());
        let mut user = user();

        let effects = ItemInteractionEffectRoomExecutor::apply(
            &mut user,
            &ItemInteractionEffect::WalkTo { x: 1, y: 0 },
            &mut mapping,
            &items,
            &[],
            true,
            0,
        );

        assert_eq!(effects, vec![RoomUserEffect::NotEnoughTickets]);
        assert!(!user.is_walking());
    }

    #[test]
    fn ignores_effects_owned_by_other_boundaries() {
        let mut mapping = RoomMapping::new(model());
        let mut user = user();

        let effects = ItemInteractionEffectRoomExecutor::apply_all(
            &mut user,
            &[
                ItemInteractionEffect::ShowProgram {
                    item_id: 1,
                    program: "open".to_owned(),
                },
                ItemInteractionEffect::SendTickets,
                ItemInteractionEffect::SavePlayer,
            ],
            &mut mapping,
            &[],
            &[],
            true,
            0,
        );

        assert!(effects.is_empty());
        assert!(!user.is_walking());
    }
}
