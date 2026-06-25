use super::game_runtime_scheduler_plan::*;

#[test]
fn plans_java_game_scheduler_worker_pool_and_tick() {
    let plan = GameRuntimeSchedulerPlan::java_default();

    assert_eq!(plan.worker_threads(), 8);
    assert_eq!(
        plan.construction_effect(),
        GameRuntimeSchedulerEffect::CreateWorkerPool { worker_threads: 8 }
    );
    assert_eq!(
        plan.schedule_game_tick_effect(),
        GameRuntimeSchedulerEffect::ScheduleFixedRate {
            task: GameRuntimeTask::GameTick,
            initial_delay_ms: 0,
            interval_ms: 1_000,
        }
    );
}

#[test]
fn plans_room_tick_tasks_at_half_second_intervals() {
    let plan = GameRuntimeSchedulerPlan::java_default();

    assert_eq!(
        plan.schedule_room_ticks_effects(42),
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
fn plans_delayed_runtime_tasks_and_room_cancellation() {
    let plan = GameRuntimeSchedulerPlan::java_default();

    assert_eq!(
        plan.schedule_bot_response_effect(7, 1_500),
        GameRuntimeSchedulerEffect::ScheduleDelayed {
            task: GameRuntimeTask::BotResponse { entity_id: 7 },
            delay_ms: 1_500,
        }
    );
    assert_eq!(
        plan.schedule_teleporter_transfer_effect(11, 22, 2_000),
        GameRuntimeSchedulerEffect::ScheduleDelayed {
            task: GameRuntimeTask::TeleporterTransfer {
                user_id: 11,
                item_id: 22,
            },
            delay_ms: 2_000,
        }
    );
    assert_eq!(
        plan.cancel_room_tasks_effect(42),
        GameRuntimeSchedulerEffect::CancelRoomTasks { room_id: 42 }
    );
}
