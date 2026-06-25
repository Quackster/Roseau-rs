use crate::game::GameRuntimeTask;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameRuntimeSchedulerEffect {
    CreateWorkerPool {
        worker_threads: usize,
    },
    ScheduleFixedRate {
        task: GameRuntimeTask,
        initial_delay_ms: u64,
        interval_ms: u64,
    },
    ScheduleDelayed {
        task: GameRuntimeTask,
        delay_ms: u64,
    },
    CancelRoomTasks {
        room_id: i32,
    },
}
