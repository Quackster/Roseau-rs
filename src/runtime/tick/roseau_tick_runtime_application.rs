use crate::runtime::{
    HostResolver, RoseauApplicationRuntime, RoseauApplicationTickExecutionReport,
    RoseauGameTickRuntimeActionPlan,
};

impl RoseauApplicationRuntime {
    pub fn apply_tick_runtime_plans(
        &mut self,
        report: &RoseauApplicationTickExecutionReport,
    ) -> Vec<RoseauGameTickRuntimeActionPlan> {
        for plan in report.runtime_plans() {
            if let RoseauGameTickRuntimeActionPlan::SyncPlayerCredits { user_id, credits } = plan {
                self.game_mut()
                    .player_manager_mut()
                    .sync_player_credits(*user_id, *credits);
            }
        }

        let network_effects = report
            .runtime_plans()
            .iter()
            .filter_map(|plan| match plan {
                RoseauGameTickRuntimeActionPlan::Network(effect) => Some(effect.clone()),
                RoseauGameTickRuntimeActionPlan::SyncPlayerCredits { .. }
                | RoseauGameTickRuntimeActionPlan::ResolveConfiguredHost { .. } => None,
            })
            .collect::<Vec<_>>();
        let unapplied_network_effects = self
            .startup_runtime_mut()
            .apply_network_effects(network_effects);
        let mut unapplied_plans = report
            .runtime_plans()
            .iter()
            .filter_map(|plan| match plan {
                RoseauGameTickRuntimeActionPlan::ResolveConfiguredHost { .. } => Some(plan.clone()),
                RoseauGameTickRuntimeActionPlan::SyncPlayerCredits { .. }
                | RoseauGameTickRuntimeActionPlan::Network(_) => None,
            })
            .collect::<Vec<_>>();

        unapplied_plans.extend(
            unapplied_network_effects
                .into_iter()
                .map(RoseauGameTickRuntimeActionPlan::Network),
        );
        unapplied_plans
    }

    pub fn apply_tick_runtime_plans_with_resolver<R: HostResolver>(
        &mut self,
        report: &RoseauApplicationTickExecutionReport,
        resolver: &R,
    ) -> Vec<RoseauGameTickRuntimeActionPlan> {
        let mut unapplied_plans = self.apply_tick_runtime_plans(report);
        let mut still_unapplied = Vec::new();

        for plan in unapplied_plans.drain(..) {
            match plan {
                RoseauGameTickRuntimeActionPlan::ResolveConfiguredHost { host } => {
                    match resolver.resolve_host(&host) {
                        Ok(address) => self.set_resolved_config_ip(address),
                        Err(_) => still_unapplied
                            .push(RoseauGameTickRuntimeActionPlan::ResolveConfiguredHost { host }),
                    }
                }
                RoseauGameTickRuntimeActionPlan::Network(effect) => {
                    still_unapplied.push(RoseauGameTickRuntimeActionPlan::Network(effect));
                }
                RoseauGameTickRuntimeActionPlan::SyncPlayerCredits { .. } => {}
            }
        }

        still_unapplied
    }
}
