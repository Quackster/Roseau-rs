use crate::messages::IncomingExecutionEffect;
use crate::runtime::{RoseauApplicationRuntime, RoseauIncomingExecutionRuntimePlan};

impl RoseauApplicationRuntime {
    pub fn collect_incoming_execution_runtime_plans(
        &self,
        effects: &[IncomingExecutionEffect],
        connection_id: i32,
    ) -> Vec<RoseauIncomingExecutionRuntimePlan> {
        RoseauIncomingExecutionRuntimePlan::collect(effects, connection_id)
    }

    pub fn apply_incoming_execution_runtime_plans(
        &mut self,
        effects: &[IncomingExecutionEffect],
        connection_id: i32,
    ) -> Vec<RoseauIncomingExecutionRuntimePlan> {
        let plans = self.collect_incoming_execution_runtime_plans(effects, connection_id);
        let network_effects = plans
            .iter()
            .map(|plan| match plan {
                RoseauIncomingExecutionRuntimePlan::Network(effect) => effect.clone(),
            })
            .collect::<Vec<_>>();

        self.startup_runtime_mut()
            .apply_network_effects(network_effects)
            .into_iter()
            .map(RoseauIncomingExecutionRuntimePlan::Network)
            .collect()
    }
}
