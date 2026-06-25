use super::room_effect_runtime_scheduler_plan::*;
use crate::game::GameRuntimeTask;

#[test]
fn maps_room_tick_effects_to_runtime_scheduler_effects() {
    let scheduler_plan = GameRuntimeSchedulerPlan::java_default();

    let effects = RoomEffectRuntimeSchedulerPlan::plan_all(
        &[
            RoomEffect::ScheduleWalkTicks,
            RoomEffect::ScheduleEventTicks,
        ],
        42,
        &scheduler_plan,
    );

    assert_eq!(
        effects,
        vec![
            GameRuntimeSchedulerEffect::ScheduleFixedRate {
                task: GameRuntimeTask::RoomWalkTick { room_id: 42 },
                initial_delay_ms: 0,
                interval_ms: 500,
            },
            GameRuntimeSchedulerEffect::ScheduleFixedRate {
                task: GameRuntimeTask::RoomEventTick { room_id: 42 },
                initial_delay_ms: 0,
                interval_ms: 500,
            },
        ]
    );
}

#[test]
fn maps_room_disposal_effects_to_runtime_scheduler_cancellation() {
    let scheduler_plan = GameRuntimeSchedulerPlan::java_default();

    let effects = RoomEffectRuntimeSchedulerPlan::plan_all(
        &[
            RoomEffect::ClearRuntimeData,
            RoomEffect::RemoveLoadedRoom { room_id: 77 },
        ],
        42,
        &scheduler_plan,
    );

    assert_eq!(
        effects,
        vec![
            GameRuntimeSchedulerEffect::CancelRoomTasks { room_id: 42 },
            GameRuntimeSchedulerEffect::CancelRoomTasks { room_id: 42 },
        ]
    );
}

#[test]
fn ignores_non_scheduler_room_effects() {
    let scheduler_plan = GameRuntimeSchedulerPlan::java_default();

    let effects = RoomEffectRuntimeSchedulerPlan::plan_all(
        &[
            RoomEffect::SendOwnerPrivileges { user_id: 7 },
            RoomEffect::RegisterEvent {
                event_name: "user_status".to_owned(),
            },
            RoomEffect::SaveRights {
                room_id: 42,
                rights: vec![7],
            },
        ],
        42,
        &scheduler_plan,
    );

    assert!(effects.is_empty());
}
