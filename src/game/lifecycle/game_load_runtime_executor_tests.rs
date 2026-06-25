use super::game_load_runtime_executor::*;
use crate::game::{GameLoadEffect, GameRuntimeSchedulerEffect, GameRuntimeTask};

#[test]
fn applies_single_load_action_into_report() {
    let report = GameLoadRuntimeExecutor::apply(&GameLoadRuntimeAction::LoadCommandManager);

    assert!(report.command_manager_loaded());
    assert!(!report.variables_loaded());
    assert!(report.scheduler_report().fixed_rate_tasks().is_empty());
}

#[test]
fn applies_all_load_runtime_actions_into_report() {
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

    let report = GameLoadRuntimeExecutor::apply_all(&actions);

    assert!(report.variables_loaded());
    assert!(report.room_manager_loaded());
    assert!(report.item_manager_loaded());
    assert!(report.catalogue_manager_loaded());
    assert!(report.command_manager_loaded());
    assert_eq!(
        report.scheduler_report().fixed_rate_tasks(),
        &[GameRuntimeSchedulerEffect::ScheduleFixedRate {
            task: GameRuntimeTask::GameTick,
            initial_delay_ms: 0,
            interval_ms: 1_000,
        }]
    );
}
