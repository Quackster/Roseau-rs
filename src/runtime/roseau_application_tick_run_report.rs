use crate::runtime::{
    RoseauApplicationTickExecutionReport, RoseauGameTickRuntimeActionPlan, RoseauServerLoopOutcome,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RoseauApplicationTickRunReport {
    execution_report: RoseauApplicationTickExecutionReport,
    server_outcome: RoseauServerLoopOutcome,
    unapplied_runtime_plans: Vec<RoseauGameTickRuntimeActionPlan>,
}

impl RoseauApplicationTickRunReport {
    pub fn new(
        execution_report: RoseauApplicationTickExecutionReport,
        server_outcome: RoseauServerLoopOutcome,
        unapplied_runtime_plans: impl Into<Vec<RoseauGameTickRuntimeActionPlan>>,
    ) -> Self {
        Self {
            execution_report,
            server_outcome,
            unapplied_runtime_plans: unapplied_runtime_plans.into(),
        }
    }

    pub fn execution_report(&self) -> &RoseauApplicationTickExecutionReport {
        &self.execution_report
    }

    pub fn server_outcome(&self) -> &RoseauServerLoopOutcome {
        &self.server_outcome
    }

    pub fn should_continue(&self) -> bool {
        self.server_outcome.should_continue()
    }

    pub fn unapplied_runtime_plans(&self) -> &[RoseauGameTickRuntimeActionPlan] {
        &self.unapplied_runtime_plans
    }
}

#[cfg(test)]
mod tests {
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
}
