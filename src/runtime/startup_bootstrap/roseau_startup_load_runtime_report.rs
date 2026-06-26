use crate::game::{
    GameLoadRuntimeAction, GameLoadRuntimeExecutor, GameLoadRuntimeReport,
    GameRuntimeSchedulerEffect,
};
use crate::runtime::RoseauApplicationRuntime;

impl RoseauApplicationRuntime {
    pub fn startup_scheduler_effects(&self) -> Vec<GameRuntimeSchedulerEffect> {
        self.game().startup_scheduler_effects()
    }

    pub fn startup_load_report(&self) -> GameLoadRuntimeReport {
        let actions = GameLoadRuntimeAction::collect(&self.game().load_effects());
        GameLoadRuntimeExecutor::apply_all(&actions)
    }
}
