use super::*;
use crate::game::GameRuntimeTask;

#[test]
fn applies_scheduler_effects_into_report() {
    let effects = vec![
        GameRuntimeSchedulerEffect::CreateWorkerPool { worker_threads: 4 },
        GameRuntimeSchedulerEffect::ScheduleFixedRate {
            task: GameRuntimeTask::GameTick,
            initial_delay_ms: 0,
            interval_ms: 1_000,
        },
        GameRuntimeSchedulerEffect::ScheduleDelayed {
            task: GameRuntimeTask::TeleporterTransfer {
                user_id: 7,
                item_id: 9,
            },
            delay_ms: 2_000,
        },
    ];

    let report = GameRuntimeSchedulerExecutor::apply_all(&effects);

    assert_eq!(report.worker_pool_threads(), Some(4));
    assert_eq!(report.fixed_rate_tasks(), &effects[1..2]);
    assert_eq!(report.delayed_tasks(), &effects[2..3]);
    assert!(report.cancelled_room_ids().is_empty());
}
