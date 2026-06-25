use crate::dao::mysql::{PlayerPasswordActionQueries, SqlExecutionPlan};
use crate::game::player::{PlayerPasswordActionOutcome, PlayerPasswordActionReport};

#[derive(Debug, Clone, PartialEq)]
pub struct MySqlPlayerPasswordActionReport {
    player_report: PlayerPasswordActionReport,
    persistence_plans: Vec<SqlExecutionPlan>,
}

impl MySqlPlayerPasswordActionReport {
    pub fn from_outcomes(
        outcomes: impl Into<Vec<PlayerPasswordActionOutcome>>,
        connection_id: i32,
        now: i64,
    ) -> Self {
        let player_report = PlayerPasswordActionReport::from_outcomes(outcomes, connection_id);
        let persistence_plans =
            PlayerPasswordActionQueries::plan_all(player_report.outcomes(), now);

        Self {
            player_report,
            persistence_plans,
        }
    }

    pub fn player_report(&self) -> &PlayerPasswordActionReport {
        &self.player_report
    }

    pub fn persistence_plans(&self) -> &[SqlExecutionPlan] {
        &self.persistence_plans
    }
}
