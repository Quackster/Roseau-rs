use super::my_sql_player_password_action_execution_report::*;
use crate::dao::mysql::SqlExecutionBatchResult;
use crate::game::player::{PlayerDetails, PlayerLoginOutcome, PlayerPasswordActionOutcome};

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "mission", "figure");
    details
}

#[test]
fn exposes_password_report_and_database_result() {
    let password_report = MySqlPlayerPasswordActionReport::from_outcomes(
        [PlayerPasswordActionOutcome::Login(
            PlayerLoginOutcome::authenticated(&details(), "secret", false, 30001, 30001, None),
        )],
        11,
        1234,
    );
    let execution_report = MySqlPlayerPasswordActionExecutionReport::new(
        password_report.clone(),
        SqlExecutionBatchResult::new([]),
    );

    assert_eq!(execution_report.password_report(), &password_report);
    assert!(execution_report.database_result().results().is_empty());
}
