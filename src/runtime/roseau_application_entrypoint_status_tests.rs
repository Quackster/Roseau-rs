use super::*;
use crate::dao::mysql::{MySqlApplicationTickExecutionReport, SqlExecutionBatchResult};
use crate::game::{
    GameLoadRuntimeAction, GameLoadRuntimeReport, GameRuntimeSchedulerEffect, GameRuntimeTask,
};
use crate::runtime::{
    RoseauApplicationTickExecutionReport, RoseauApplicationTickRunReport, RoseauServerLoopOutcome,
    RoseauStartupRuntimeError, RoseauStartupRuntimeStatus,
};

fn ready_prepare() -> RoseauApplicationPrepareReadiness {
    RoseauApplicationPrepareReadiness::new(
        true,
        Some(
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
            .readiness(),
        ),
        Some(RoseauStartupRuntimeStatus::Ready {
            bind_addresses: vec!["127.0.0.1:37120".to_owned()],
            active_connections: 0,
        }),
    )
}

#[test]
fn reports_prepared_entrypoint_before_loop_runs() {
    let status = RoseauApplicationEntrypointStatus::new(ready_prepare(), None);

    assert!(status.ready());
    assert!(status.prepare_readiness().ready());
    assert!(!status.loop_ran());
    assert_eq!(status.completed_ticks(), 0);
    assert_eq!(status.loop_stopped(), None);
}

#[test]
fn reports_loop_execution_and_stop_state() {
    let execution_report = RoseauApplicationTickExecutionReport::from_database_report(
        MySqlApplicationTickExecutionReport::new(SqlExecutionBatchResult::new([]), []),
        "127.0.0.1",
        &crate::game::player::PlayerManager::new(vec![]),
    );
    let tick_report = RoseauApplicationTickRunReport::new(
        execution_report,
        RoseauServerLoopOutcome::Stop {
            error: RoseauStartupRuntimeError::NotListening,
        },
        [],
    );
    let loop_report = RoseauApplicationLoopReport::new([tick_report], true);

    let status = RoseauApplicationEntrypointStatus::new(ready_prepare(), Some(&loop_report));

    assert!(status.ready());
    assert!(status.loop_ran());
    assert_eq!(status.completed_ticks(), 1);
    assert_eq!(status.loop_stopped(), Some(true));
}
