use crate::game::{GameLoadEffect, GameRuntimeSchedulerEffect, GameRuntimeTask};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameLoadRuntimeAction {
    LoadVariables,
    LoadRoomManager,
    LoadItemManager,
    LoadCatalogueManager,
    LoadCommandManager,
    Scheduler(GameRuntimeSchedulerEffect),
}

impl GameLoadRuntimeAction {
    pub fn from_effect(effect: &GameLoadEffect) -> Self {
        match effect {
            GameLoadEffect::LoadVariables => Self::LoadVariables,
            GameLoadEffect::LoadRoomManager => Self::LoadRoomManager,
            GameLoadEffect::LoadItemManager => Self::LoadItemManager,
            GameLoadEffect::LoadCatalogueManager => Self::LoadCatalogueManager,
            GameLoadEffect::LoadCommandManager => Self::LoadCommandManager,
            GameLoadEffect::ScheduleGameTick {
                initial_delay_secs,
                interval_secs,
            } => Self::Scheduler(GameRuntimeSchedulerEffect::ScheduleFixedRate {
                task: GameRuntimeTask::GameTick,
                initial_delay_ms: initial_delay_secs.saturating_mul(1_000),
                interval_ms: interval_secs.saturating_mul(1_000),
            }),
        }
    }

    pub fn collect(effects: &[GameLoadEffect]) -> Vec<Self> {
        effects.iter().map(Self::from_effect).collect()
    }
}

#[cfg(test)]
#[path = "game_load_runtime_action_tests.rs"]
mod tests;
