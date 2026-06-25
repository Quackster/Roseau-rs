use super::item_interaction_runtime_plan::*;
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
