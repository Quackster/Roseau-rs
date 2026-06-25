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

#[cfg(test)]
mod tests {
    use super::*;

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
}
