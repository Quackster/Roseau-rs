use crate::game::{GameLoadRuntimeAction, GameLoadRuntimeReport};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GameLoadRuntimeExecutor;

impl GameLoadRuntimeExecutor {
    pub fn apply(action: &GameLoadRuntimeAction) -> GameLoadRuntimeReport {
        let mut report = GameLoadRuntimeReport::new();
        report.record(action);
        report
    }

    pub fn apply_all(actions: &[GameLoadRuntimeAction]) -> GameLoadRuntimeReport {
        let mut report = GameLoadRuntimeReport::new();

        for action in actions {
            report.record(action);
        }

        report
    }
}
