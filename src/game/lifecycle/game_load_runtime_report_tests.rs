use super::*;
use crate::game::{GameRuntimeSchedulerEffect, GameRuntimeTask};

#[test]
fn defaults_to_unloaded_managers_and_empty_scheduler_report() {
    let report = GameLoadRuntimeReport::new();

    assert!(!report.variables_loaded());
    assert!(!report.room_manager_loaded());
    assert!(!report.item_manager_loaded());
    assert!(!report.catalogue_manager_loaded());
    assert!(!report.command_manager_loaded());
    assert!(report.scheduler_report().fixed_rate_tasks().is_empty());
    assert!(report.scheduler_report().delayed_tasks().is_empty());
    assert!(report.scheduler_report().cancelled_room_ids().is_empty());
}

#[test]
fn records_manager_load_actions_and_scheduler_work() {
    let scheduler_effect = GameRuntimeSchedulerEffect::ScheduleFixedRate {
        task: GameRuntimeTask::GameTick,
        initial_delay_ms: 0,
        interval_ms: 1_000,
    };
    let actions = vec![
        GameLoadRuntimeAction::LoadVariables,
        GameLoadRuntimeAction::LoadRoomManager,
        GameLoadRuntimeAction::LoadItemManager,
        GameLoadRuntimeAction::LoadCatalogueManager,
        GameLoadRuntimeAction::LoadCommandManager,
        GameLoadRuntimeAction::Scheduler(scheduler_effect.clone()),
    ];

    let report = GameLoadRuntimeReport::from_actions(&actions);

    assert!(report.variables_loaded());
    assert!(report.room_manager_loaded());
    assert!(report.item_manager_loaded());
    assert!(report.catalogue_manager_loaded());
    assert!(report.command_manager_loaded());
    assert_eq!(
        report.scheduler_report().fixed_rate_tasks(),
        &[scheduler_effect]
    );
    assert!(report.scheduler_report().delayed_tasks().is_empty());
    assert!(report.readiness().ready());
}

#[test]
fn exposes_readiness_for_incomplete_load_reports() {
    let report = GameLoadRuntimeReport::from_actions(&[GameLoadRuntimeAction::LoadVariables]);

    let readiness = report.readiness();

    assert!(!readiness.ready());
    assert_eq!(
        readiness.missing_steps(),
        &[
            "room_manager",
            "item_manager",
            "catalogue_manager",
            "command_manager"
        ]
    );
    assert!(!readiness.game_tick_scheduled());
}
