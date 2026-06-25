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
mod tests {
    use super::*;

    #[test]
    fn maps_game_load_effects_to_runtime_actions() {
        let actions = GameLoadRuntimeAction::collect(&[
            GameLoadEffect::LoadVariables,
            GameLoadEffect::LoadRoomManager,
            GameLoadEffect::LoadItemManager,
            GameLoadEffect::LoadCatalogueManager,
            GameLoadEffect::LoadCommandManager,
            GameLoadEffect::ScheduleGameTick {
                initial_delay_secs: 0,
                interval_secs: 1,
            },
        ]);

        assert_eq!(
            actions,
            vec![
                GameLoadRuntimeAction::LoadVariables,
                GameLoadRuntimeAction::LoadRoomManager,
                GameLoadRuntimeAction::LoadItemManager,
                GameLoadRuntimeAction::LoadCatalogueManager,
                GameLoadRuntimeAction::LoadCommandManager,
                GameLoadRuntimeAction::Scheduler(GameRuntimeSchedulerEffect::ScheduleFixedRate {
                    task: GameRuntimeTask::GameTick,
                    initial_delay_ms: 0,
                    interval_ms: 1_000,
                }),
            ]
        );
    }

    #[test]
    fn converts_second_based_tick_schedule_to_milliseconds() {
        let action = GameLoadRuntimeAction::from_effect(&GameLoadEffect::ScheduleGameTick {
            initial_delay_secs: 2,
            interval_secs: 3,
        });

        assert_eq!(
            action,
            GameLoadRuntimeAction::Scheduler(GameRuntimeSchedulerEffect::ScheduleFixedRate {
                task: GameRuntimeTask::GameTick,
                initial_delay_ms: 2_000,
                interval_ms: 3_000,
            })
        );
    }
}
