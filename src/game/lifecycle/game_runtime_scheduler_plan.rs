use crate::game::{GameRuntimeSchedulerEffect, GameRuntimeTask};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameRuntimeSchedulerPlan {
    worker_threads: usize,
}

impl GameRuntimeSchedulerPlan {
    pub const DEFAULT_WORKER_THREADS: usize = 8;

    pub fn new(worker_threads: usize) -> Self {
        Self { worker_threads }
    }

    pub fn java_default() -> Self {
        Self::new(Self::DEFAULT_WORKER_THREADS)
    }

    pub fn worker_threads(&self) -> usize {
        self.worker_threads
    }

    pub fn construction_effect(&self) -> GameRuntimeSchedulerEffect {
        GameRuntimeSchedulerEffect::CreateWorkerPool {
            worker_threads: self.worker_threads,
        }
    }

    pub fn schedule_game_tick_effect(&self) -> GameRuntimeSchedulerEffect {
        GameRuntimeSchedulerEffect::ScheduleFixedRate {
            task: GameRuntimeTask::GameTick,
            initial_delay_ms: 0,
            interval_ms: 1_000,
        }
    }

    pub fn schedule_room_ticks_effects(&self, room_id: i32) -> Vec<GameRuntimeSchedulerEffect> {
        vec![
            GameRuntimeSchedulerEffect::ScheduleFixedRate {
                task: GameRuntimeTask::RoomWalkTick { room_id },
                initial_delay_ms: 0,
                interval_ms: 500,
            },
            GameRuntimeSchedulerEffect::ScheduleFixedRate {
                task: GameRuntimeTask::RoomEventTick { room_id },
                initial_delay_ms: 0,
                interval_ms: 500,
            },
        ]
    }

    pub fn schedule_bot_response_effect(
        &self,
        entity_id: i32,
        delay_ms: u64,
    ) -> GameRuntimeSchedulerEffect {
        GameRuntimeSchedulerEffect::ScheduleDelayed {
            task: GameRuntimeTask::BotResponse { entity_id },
            delay_ms,
        }
    }

    pub fn schedule_teleporter_transfer_effect(
        &self,
        user_id: i32,
        item_id: i32,
        delay_ms: u64,
    ) -> GameRuntimeSchedulerEffect {
        GameRuntimeSchedulerEffect::ScheduleDelayed {
            task: GameRuntimeTask::TeleporterTransfer { user_id, item_id },
            delay_ms,
        }
    }

    pub fn cancel_room_tasks_effect(&self, room_id: i32) -> GameRuntimeSchedulerEffect {
        GameRuntimeSchedulerEffect::CancelRoomTasks { room_id }
    }
}

impl Default for GameRuntimeSchedulerPlan {
    fn default() -> Self {
        Self::java_default()
    }
}
