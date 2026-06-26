use super::*;
use crate::game::{GameLoadRuntimeAction, GameRuntimeSchedulerEffect, GameRuntimeTask};

#[test]
fn reports_complete_load_readiness() {
    let report = GameLoadRuntimeReport::from_actions(&[
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
    ]);

    let readiness = GameLoadReadiness::from_report(&report);

    assert!(readiness.ready());
    assert!(readiness.missing_steps().is_empty());
    assert!(readiness.game_tick_scheduled());
}

#[test]
fn reports_missing_load_steps_and_tick_schedule() {
    let report = GameLoadRuntimeReport::from_actions(&[
        GameLoadRuntimeAction::LoadVariables,
        GameLoadRuntimeAction::LoadCommandManager,
    ]);

    let readiness = GameLoadReadiness::from_report(&report);

    assert!(!readiness.ready());
    assert_eq!(
        readiness.missing_steps(),
        &["room_manager", "item_manager", "catalogue_manager"]
    );
    assert!(!readiness.game_tick_scheduled());
}
