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
mod tests {
    use super::*;
    use crate::game::room::model::Position;

    #[test]
    fn maps_scheduled_item_effects_to_runtime_work() {
        let nested = vec![
            ItemInteractionEffect::SetCanWalk { can_walk: true },
            ItemInteractionEffect::WalkTo { x: 3, y: 4 },
        ];

        let effects = ItemInteractionRuntimePlan::plan(
            &ItemInteractionEffect::Schedule {
                delay_ms: 800,
                effects: nested.clone(),
            },
            7,
            5,
        );

        assert_eq!(
            effects,
            vec![ItemInteractionRuntimeEffect::ScheduleEffects {
                user_id: 7,
                delay_ms: 800,
                effects: nested,
            }]
        );
    }

    #[test]
    fn maps_teleporter_room_transfer_effects() {
        let effects = ItemInteractionRuntimePlan::plan_all(
            &[
                ItemInteractionEffect::LeaveRoom { room_id: 10 },
                ItemInteractionEffect::LoadRoom {
                    room_id: 20,
                    position: Position::with_rotation(5, 6, 0.0, 2),
                    rotation: 2,
                },
            ],
            7,
            5,
        );

        assert_eq!(
            effects,
            vec![
                ItemInteractionRuntimeEffect::LeaveRoom {
                    user_id: 7,
                    room_id: 10,
                },
                ItemInteractionRuntimeEffect::LoadRoom {
                    user_id: 7,
                    room_id: 20,
                    position: Position::with_rotation(5, 6, 0.0, 2),
                    rotation: 2,
                },
            ]
        );
    }

    #[test]
    fn ignores_state_network_and_persistence_effects() {
        let effects = ItemInteractionRuntimePlan::plan_all(
            &[
                ItemInteractionEffect::SetCanWalk { can_walk: false },
                ItemInteractionEffect::SendDoorOut { item_id: 1 },
            ],
            7,
            5,
        );

        assert!(effects.is_empty());
    }

    #[test]
    fn maps_saved_ticket_decrement_to_runtime_ticket_sync() {
        let effects = ItemInteractionRuntimePlan::plan_all(
            &[
                ItemInteractionEffect::DecrementTickets { amount: 1 },
                ItemInteractionEffect::SendTickets,
                ItemInteractionEffect::SavePlayer,
            ],
            7,
            5,
        );

        assert_eq!(
            effects,
            vec![ItemInteractionRuntimeEffect::SyncPlayerTickets {
                user_id: 7,
                tickets: 4,
            }]
        );
    }
}
