use crate::dao::mysql::{PlayerEffectQueries, SqlExecutionPlan};
use crate::game::player::{PlayerPasswordActionEffectPlan, PlayerPasswordActionOutcome};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerPasswordActionQueries;

impl PlayerPasswordActionQueries {
    pub fn plan(outcome: &PlayerPasswordActionOutcome, now: i64) -> Vec<SqlExecutionPlan> {
        PlayerPasswordActionEffectPlan::plan(outcome)
            .iter()
            .filter_map(|effect| PlayerEffectQueries::plan(effect, now))
            .collect()
    }

    pub fn plan_all(outcomes: &[PlayerPasswordActionOutcome], now: i64) -> Vec<SqlExecutionPlan> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, now))
            .collect()
    }
}

#[cfg(test)]
#[path = "player_password_action_queries_tests.rs"]
mod tests;
