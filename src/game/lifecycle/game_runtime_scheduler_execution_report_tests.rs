use super::*;

#[test]
fn records_scheduler_effects_and_cancels_room_tasks() {
    let mut report = GameRuntimeSchedulerExecutionReport::new();
    let room_walk = GameRuntimeSchedulerEffect::ScheduleFixedRate {
        task: GameRuntimeTask::RoomWalkTick { room_id: 42 },
        initial_delay_ms: 0,
        interval_ms: 500,
    };
    let room_event = GameRuntimeSchedulerEffect::ScheduleFixedRate {
        task: GameRuntimeTask::RoomEventTick { room_id: 42 },
        initial_delay_ms: 0,
        interval_ms: 500,
    };
    let game_tick = GameRuntimeSchedulerEffect::ScheduleFixedRate {
        task: GameRuntimeTask::GameTick,
        initial_delay_ms: 0,
        interval_ms: 1_000,
    };
    let bot_response = GameRuntimeSchedulerEffect::ScheduleDelayed {
        task: GameRuntimeTask::BotResponse { entity_id: 7 },
        delay_ms: 1_500,
    };

    for effect in [
        GameRuntimeSchedulerEffect::CreateWorkerPool { worker_threads: 8 },
        room_walk,
        room_event,
        game_tick.clone(),
        bot_response.clone(),
        GameRuntimeSchedulerEffect::CancelRoomTasks { room_id: 42 },
    ] {
        report.record(&effect);
    }

    assert_eq!(report.worker_pool_threads(), Some(8));
    assert_eq!(report.fixed_rate_tasks(), &[game_tick]);
    assert_eq!(report.delayed_tasks(), &[bot_response]);
    assert_eq!(report.cancelled_room_ids(), &[42]);
}
