use super::my_sql_player_password_action_executor::*;
use crate::dao::mysql::{SqlExecutionPlan, SqlExecutionResult};
use crate::game::player::{
    PlayerDetails, PlayerLoginOutcome, PlayerPasswordActionOutcome, PlayerRegistrationOutcome,
};
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, Default)]
struct RecordingExecutor {
    plans: RefCell<Vec<SqlExecutionPlan>>,
    results: RefCell<VecDeque<Result<SqlExecutionResult, DaoError>>>,
}

impl RecordingExecutor {
    fn push_result(&self, result: SqlExecutionResult) {
        self.results.borrow_mut().push_back(Ok(result));
    }

    fn plans(&self) -> Vec<SqlExecutionPlan> {
        self.plans.borrow().clone()
    }
}

impl SqlExecutor for RecordingExecutor {
    fn execute(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        self.plans.borrow_mut().push(plan);
        self.results
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| Err(DaoError::new("missing executor result")))
    }
}

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "mission", "figure");
    details
}

#[test]
fn executes_persistent_password_action_plans() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let password_executor = MySqlPlayerPasswordActionExecutor::new(executor);
    let report = MySqlPlayerPasswordActionReport::from_outcomes(
        [PlayerPasswordActionOutcome::Login(
            PlayerLoginOutcome::authenticated(&details(), "secret", false, 30001, 30001, Some(42)),
        )],
        11,
        1234,
    );

    let result = password_executor.execute_report(&report).unwrap();
    let executor = password_executor.into_executor();

    assert_eq!(result.results(), &[SqlExecutionResult::AffectedRows(1)]);
    assert_eq!(executor.plans().len(), 1);
    assert_eq!(
        executor.plans()[0].sql(),
        "UPDATE users SET last_online = ? WHERE id = ?"
    );
}

#[test]
fn returns_empty_batch_for_non_persistent_password_actions() {
    let executor = RecordingExecutor::default();
    let password_executor = MySqlPlayerPasswordActionExecutor::new(executor);
    let report = MySqlPlayerPasswordActionReport::from_outcomes(
        [PlayerPasswordActionOutcome::Registration(
            PlayerRegistrationOutcome::Created,
        )],
        11,
        1234,
    );

    let result = password_executor.execute_report(&report).unwrap();
    let executor = password_executor.into_executor();

    assert!(result.results().is_empty());
    assert!(executor.plans().is_empty());
}

#[test]
fn returns_execution_report_with_database_result_and_runtime_follow_up() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let password_executor = MySqlPlayerPasswordActionExecutor::new(executor);
    let report = MySqlPlayerPasswordActionReport::from_outcomes(
        [PlayerPasswordActionOutcome::Login(
            PlayerLoginOutcome::authenticated(&details(), "secret", false, 30001, 30001, Some(42)),
        )],
        11,
        1234,
    );

    let execution_report = password_executor.execute_password_report(report).unwrap();

    assert_eq!(
        execution_report.database_result().results(),
        &[SqlExecutionResult::AffectedRows(1)]
    );
    assert_eq!(
        execution_report
            .password_report()
            .player_report()
            .player_effects()
            .len(),
        2
    );
}
