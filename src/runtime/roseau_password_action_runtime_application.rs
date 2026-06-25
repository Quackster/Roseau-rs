use crate::dao::mysql::MySqlPlayerPasswordActionExecutionReport;
use crate::runtime::{RoseauApplicationRuntime, RoseauPasswordActionRuntimePlan};

impl RoseauApplicationRuntime {
    pub fn collect_password_action_runtime_plans(
        &self,
        report: &MySqlPlayerPasswordActionExecutionReport,
    ) -> Vec<RoseauPasswordActionRuntimePlan> {
        RoseauPasswordActionRuntimePlan::collect(report, self.game().player_manager())
    }

    pub fn apply_password_action_runtime_plans(
        &mut self,
        report: &MySqlPlayerPasswordActionExecutionReport,
    ) -> Vec<RoseauPasswordActionRuntimePlan> {
        let plans = self.collect_password_action_runtime_plans(report);
        let network_effects = plans
            .iter()
            .map(|plan| match plan {
                RoseauPasswordActionRuntimePlan::Network(effect) => effect.clone(),
            })
            .collect::<Vec<_>>();

        self.startup_runtime_mut()
            .apply_network_effects(network_effects)
            .into_iter()
            .map(RoseauPasswordActionRuntimePlan::Network)
            .collect()
    }
}
