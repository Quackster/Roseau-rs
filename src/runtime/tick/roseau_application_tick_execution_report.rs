use crate::dao::mysql::MySqlApplicationTickExecutionReport;
use crate::game::player::PlayerManager;
use crate::runtime::RoseauGameTickRuntimeActionPlan;

#[derive(Debug, Clone, PartialEq)]
pub struct RoseauApplicationTickExecutionReport {
    database_report: MySqlApplicationTickExecutionReport,
    runtime_plans: Vec<RoseauGameTickRuntimeActionPlan>,
}

impl RoseauApplicationTickExecutionReport {
    pub fn from_database_report(
        database_report: MySqlApplicationTickExecutionReport,
        raw_config_ip: &str,
        player_manager: &PlayerManager,
    ) -> Self {
        let runtime_actions = database_report.runtime_actions();
        let runtime_plans = RoseauGameTickRuntimeActionPlan::collect(
            &runtime_actions,
            raw_config_ip,
            player_manager,
        );

        Self {
            database_report,
            runtime_plans,
        }
    }

    pub fn database_report(&self) -> &MySqlApplicationTickExecutionReport {
        &self.database_report
    }

    pub fn runtime_plans(&self) -> &[RoseauGameTickRuntimeActionPlan] {
        &self.runtime_plans
    }
}

#[cfg(test)]
#[path = "roseau_application_tick_execution_report_tests.rs"]
mod tests;
