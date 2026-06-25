use crate::game::{GameRuntimeSchedulerEffect, GameRuntimeSchedulerExecutionReport};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GameRuntimeSchedulerExecutor;

impl GameRuntimeSchedulerExecutor {
    pub fn apply(effect: &GameRuntimeSchedulerEffect) -> GameRuntimeSchedulerExecutionReport {
        let mut report = GameRuntimeSchedulerExecutionReport::new();
        report.record(effect);
        report
    }

    pub fn apply_all(
        effects: &[GameRuntimeSchedulerEffect],
    ) -> GameRuntimeSchedulerExecutionReport {
        let mut report = GameRuntimeSchedulerExecutionReport::new();

        for effect in effects {
            report.record(effect);
        }

        report
    }
}
