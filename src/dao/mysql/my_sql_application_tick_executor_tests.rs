use super::my_sql_application_tick_executor::*;
use crate::dao::mysql::{SqlExecutionPlan, SqlExecutionResult};
use crate::dao::DaoError;
use crate::game::GameTickEffect;
use crate::runtime::{RoseauServerLoopOutcome, RoseauStartupRuntimeError};
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

#[test]
fn persists_application_tick_game_effects() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let application_tick_executor = MySqlApplicationTickExecutor::new(executor);
    let outcome = RoseauApplicationTickOutcome::new(
        [
            GameTickEffect::AwardCredits {
                user_id: 11,
                amount: 10,
                new_balance: 40,
            },
            GameTickEffect::SavePlayer { user_id: 11 },
        ],
        RoseauServerLoopOutcome::Stop {
            error: RoseauStartupRuntimeError::NotListening,
        },
    );

    let result = application_tick_executor.execute_tick(&outcome).unwrap();
    let executor = application_tick_executor.into_executor();

    assert_eq!(result.results(), &[SqlExecutionResult::AffectedRows(1)]);
    assert_eq!(executor.plans().len(), 1);
    assert_eq!(
        executor.plans()[0].sql(),
        "UPDATE users SET credits = ? WHERE id = ?"
    );
}

#[test]
fn skips_application_tick_without_persistent_effects() {
    let executor = RecordingExecutor::default();
    let application_tick_executor = MySqlApplicationTickExecutor::new(executor);
    let outcome = RoseauApplicationTickOutcome::new(
        [GameTickEffect::ResolveServerIp],
        RoseauServerLoopOutcome::Stop {
            error: RoseauStartupRuntimeError::NotListening,
        },
    );

    let result = application_tick_executor.execute_tick(&outcome).unwrap();
    let executor = application_tick_executor.into_executor();

    assert!(result.results().is_empty());
    assert!(executor.plans().is_empty());
}

#[test]
fn reports_database_result_and_runtime_effects() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let application_tick_executor = MySqlApplicationTickExecutor::new(executor);
    let outcome = RoseauApplicationTickOutcome::new(
        [
            GameTickEffect::AwardCredits {
                user_id: 11,
                amount: 10,
                new_balance: 40,
            },
            GameTickEffect::ResolveServerIp,
            GameTickEffect::KickAfkUser { user_id: 12 },
        ],
        RoseauServerLoopOutcome::Stop {
            error: RoseauStartupRuntimeError::NotListening,
        },
    );

    let report = application_tick_executor
        .execute_tick_report(&outcome)
        .unwrap();

    assert_eq!(
        report.database_result().results(),
        &[SqlExecutionResult::AffectedRows(1)]
    );
    assert_eq!(
        report.runtime_effects(),
        &[
            GameTickEffect::AwardCredits {
                user_id: 11,
                amount: 10,
                new_balance: 40,
            },
            GameTickEffect::ResolveServerIp,
            GameTickEffect::KickAfkUser { user_id: 12 },
        ]
    );
}
