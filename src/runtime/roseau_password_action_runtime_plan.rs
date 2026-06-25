use crate::dao::mysql::MySqlPlayerPasswordActionExecutionReport;
use crate::game::player::{PlayerEffectNetworkPlan, PlayerManager};
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoseauPasswordActionRuntimePlan {
    Network(PlayerNetworkEffect),
}

impl RoseauPasswordActionRuntimePlan {
    pub fn collect(
        report: &MySqlPlayerPasswordActionExecutionReport,
        player_manager: &PlayerManager,
    ) -> Vec<Self> {
        let player_report = report.password_report().player_report();
        let mut network_effects = player_report.network_effects().to_vec();
        network_effects.extend(PlayerEffectNetworkPlan::plan_all(
            player_report.player_effects(),
            player_manager,
        ));

        network_effects.into_iter().map(Self::Network).collect()
    }
}

#[cfg(test)]
#[path = "roseau_password_action_runtime_plan_tests.rs"]
mod tests;
