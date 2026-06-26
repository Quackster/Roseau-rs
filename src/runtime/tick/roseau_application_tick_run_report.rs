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
#[path = "roseau_application_tick_run_report_tests.rs"]
mod tests;
