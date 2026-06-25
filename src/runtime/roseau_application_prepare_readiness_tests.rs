use super::roseau_application_prepare_readiness::*;
use crate::game::{
    GameLoadRuntimeAction, GameLoadRuntimeReport, GameRuntimeSchedulerEffect, GameRuntimeTask,
};

fn loaded_game_readiness() -> GameLoadReadiness {
    GameLoadRuntimeReport::from_actions(&[
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
    ])
    .readiness()
}

#[test]
fn ready_when_database_game_load_and_startup_are_ready() {
    let readiness = RoseauApplicationPrepareReadiness::new(
        true,
        Some(loaded_game_readiness()),
        Some(RoseauStartupRuntimeStatus::Ready {
            bind_addresses: vec!["127.0.0.1:37120".to_owned()],
            active_connections: 0,
        }),
    );

    assert!(readiness.ready());
    assert!(readiness.database_connected());
    assert!(readiness.game_load_readiness().unwrap().ready());
    assert!(readiness.startup_status().unwrap().ready());
}

#[test]
fn not_ready_without_database_runtime_or_listening_startup() {
    let missing_runtime = RoseauApplicationPrepareReadiness::new(false, None, None);

    assert!(!missing_runtime.ready());
    assert!(!missing_runtime.database_connected());
    assert!(missing_runtime.game_load_readiness().is_none());
    assert!(missing_runtime.startup_status().is_none());

    let bind_failed = RoseauApplicationPrepareReadiness::new(
        true,
        Some(loaded_game_readiness()),
        Some(RoseauStartupRuntimeStatus::BindFailed {
            bind_addresses: vec!["127.0.0.1:37120".to_owned()],
            failed_address: "127.0.0.1:37120".to_owned(),
        }),
    );

    assert!(!bind_failed.ready());
    assert!(!bind_failed.startup_status().unwrap().ready());
}
