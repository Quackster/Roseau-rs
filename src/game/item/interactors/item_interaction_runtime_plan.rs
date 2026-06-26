use crate::game::item::interactors::{ItemInteractionEffect, ItemInteractionRuntimeEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemInteractionRuntimePlan;

impl ItemInteractionRuntimePlan {
    pub fn plan(
        effect: &ItemInteractionEffect,
        user_id: i32,
        _current_tickets: i32,
    ) -> Vec<ItemInteractionRuntimeEffect> {
        match effect {
            ItemInteractionEffect::Schedule { delay_ms, effects } => {
                vec![ItemInteractionRuntimeEffect::ScheduleEffects {
                    user_id,
                    delay_ms: *delay_ms,
                    effects: effects.clone(),
                }]
            }
            ItemInteractionEffect::LoadRoom {
                room_id,
                position,
                rotation,
            } => vec![ItemInteractionRuntimeEffect::LoadRoom {
                user_id,
                room_id: *room_id,
                position: *position,
                rotation: *rotation,
            }],
            ItemInteractionEffect::LeaveRoom { room_id } => {
                vec![ItemInteractionRuntimeEffect::LeaveRoom {
                    user_id,
                    room_id: *room_id,
                }]
            }
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
            | ItemInteractionEffect::SetItemCustomData { .. }
            | ItemInteractionEffect::UpdateItemStatus { .. } => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[ItemInteractionEffect],
        user_id: i32,
        current_tickets: i32,
    ) -> Vec<ItemInteractionRuntimeEffect> {
        let mut plans = Vec::new();
        let ticket_delta: i32 = effects
            .iter()
            .filter_map(|effect| match effect {
                ItemInteractionEffect::DecrementTickets { amount } => Some(*amount),
                _ => None,
            })
            .sum();
        let should_save_player = effects
            .iter()
            .any(|effect| matches!(effect, ItemInteractionEffect::SavePlayer));

        if should_save_player && ticket_delta != 0 {
            plans.push(ItemInteractionRuntimeEffect::SyncPlayerTickets {
                user_id,
                tickets: current_tickets - ticket_delta,
            });
        }

        plans.extend(
            effects
                .iter()
                .flat_map(|effect| Self::plan(effect, user_id, current_tickets)),
        );
        plans
    }
}

#[cfg(test)]
#[path = "item_interaction_runtime_plan_tests.rs"]
mod tests;
