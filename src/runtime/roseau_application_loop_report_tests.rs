use super::roseau_application_loop_report::*;
use crate::dao::mysql::{MySqlApplicationTickExecutionReport, SqlExecutionBatchResult};
use crate::runtime::{
    RoseauApplicationTickExecutionReport, RoseauApplicationTickRunReport, RoseauServerLoopOutcome,
    RoseauStartupRuntimeError,
};

#[test]
fn exposes_tick_reports_and_loop_decision() {
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

    let report = RoseauApplicationLoopReport::new([tick_report], true);

    assert_eq!(report.completed_ticks(), 1);
    assert!(report.stopped());
    assert!(!report.should_continue());
}
