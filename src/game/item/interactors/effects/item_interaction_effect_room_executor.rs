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
