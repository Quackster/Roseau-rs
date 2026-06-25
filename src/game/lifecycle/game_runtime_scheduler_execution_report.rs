use crate::game::{GameRuntimeSchedulerEffect, GameRuntimeTask};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameRuntimeSchedulerExecutionReport {
    worker_pool_threads: Option<usize>,
    fixed_rate_tasks: Vec<GameRuntimeSchedulerEffect>,
    delayed_tasks: Vec<GameRuntimeSchedulerEffect>,
    cancelled_room_ids: Vec<i32>,
}

impl GameRuntimeSchedulerExecutionReport {
    pub fn new() -> Self {
        Self {
            worker_pool_threads: None,
            fixed_rate_tasks: Vec::new(),
            delayed_tasks: Vec::new(),
            cancelled_room_ids: Vec::new(),
        }
    }

    pub fn record(&mut self, effect: &GameRuntimeSchedulerEffect) {
        match effect {
            GameRuntimeSchedulerEffect::CreateWorkerPool { worker_threads } => {
                self.worker_pool_threads = Some(*worker_threads);
            }
            GameRuntimeSchedulerEffect::ScheduleFixedRate { .. } => {
                self.fixed_rate_tasks.push(effect.clone());
            }
            GameRuntimeSchedulerEffect::ScheduleDelayed { .. } => {
                self.delayed_tasks.push(effect.clone());
            }
            GameRuntimeSchedulerEffect::CancelRoomTasks { room_id } => {
                self.cancelled_room_ids.push(*room_id);
                self.fixed_rate_tasks
                    .retain(|task| !Self::is_room_task(task, *room_id));
                self.delayed_tasks
                    .retain(|task| !Self::is_room_task(task, *room_id));
            }
        }
    }

    pub fn worker_pool_threads(&self) -> Option<usize> {
        self.worker_pool_threads
    }

    pub fn fixed_rate_tasks(&self) -> &[GameRuntimeSchedulerEffect] {
        &self.fixed_rate_tasks
    }

    pub fn delayed_tasks(&self) -> &[GameRuntimeSchedulerEffect] {
        &self.delayed_tasks
    }

    pub fn cancelled_room_ids(&self) -> &[i32] {
        &self.cancelled_room_ids
    }

    fn is_room_task(effect: &GameRuntimeSchedulerEffect, room_id: i32) -> bool {
        match effect {
            GameRuntimeSchedulerEffect::ScheduleFixedRate { task, .. }
            | GameRuntimeSchedulerEffect::ScheduleDelayed { task, .. } => {
                Self::task_room_id(task) == Some(room_id)
            }
            GameRuntimeSchedulerEffect::CreateWorkerPool { .. }
            | GameRuntimeSchedulerEffect::CancelRoomTasks { .. } => false,
        }
    }

    fn task_room_id(task: &GameRuntimeTask) -> Option<i32> {
        match task {
            GameRuntimeTask::RoomWalkTick { room_id }
            | GameRuntimeTask::RoomEventTick { room_id } => Some(*room_id),
            GameRuntimeTask::GameTick
            | GameRuntimeTask::BotResponse { .. }
            | GameRuntimeTask::TeleporterTransfer { .. } => None,
        }
    }
}

impl Default for GameRuntimeSchedulerExecutionReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "game_runtime_scheduler_execution_report_tests.rs"]
mod tests;
