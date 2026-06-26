use super::*;
use crate::dao::mysql::{MySqlApplicationTickExecutionReport, SqlExecutionBatchResult};
use crate::game::GameTickEffect;
use crate::server::{TcpServerAcceptOutcome, TcpServerTickOutcome};

#[test]
fn exposes_execution_report_server_outcome_and_unapplied_runtime_plans() {
    let execution_report = RoseauApplicationTickExecutionReport::from_database_report(
        MySqlApplicationTickExecutionReport::new(
            SqlExecutionBatchResult::new([]),
            [GameTickEffect::ResolveServerIp],
        ),
        "roseau.local",
        &crate::game::player::PlayerManager::new(vec![]),
    );

    let report = RoseauApplicationTickRunReport::new(
        execution_report,
        RoseauServerLoopOutcome::Continue {
            tick: TcpServerTickOutcome::new(TcpServerAcceptOutcome::Idle, [], []),
        },
        [RoseauGameTickRuntimeActionPlan::ResolveConfiguredHost {
            host: "roseau.local".to_owned(),
        }],
    );

    assert!(report.should_continue());
    assert!(report.server_outcome().tick().is_some());
    assert_eq!(
        report.unapplied_runtime_plans(),
        &[RoseauGameTickRuntimeActionPlan::ResolveConfiguredHost {
            host: "roseau.local".to_owned(),
        }]
    );
    assert!(report
        .execution_report()
        .database_report()
        .database_result()
        .results()
        .is_empty());
}
