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
mod tests {
    use super::*;
    use crate::dao::mysql::SqlExecutionBatchResult;
    use crate::game::player::{PlayerDetails, PlayerSession};
    use crate::game::GameTickEffect;
    use crate::server::PlayerNetworkEffect;

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    #[test]
    fn plans_runtime_actions_from_database_tick_report() {
        let database_report = MySqlApplicationTickExecutionReport::new(
            SqlExecutionBatchResult::new([]),
            [
                GameTickEffect::ResolveServerIp,
                GameTickEffect::KickAfkUser { user_id: 7 },
            ],
        );
        let mut player_manager = PlayerManager::new(vec![]);
        player_manager.insert(PlayerSession::new(21, 37120, details(7, "alice")));
        player_manager.insert(PlayerSession::new(22, 37120, details(8, "bob")));

        let report = RoseauApplicationTickExecutionReport::from_database_report(
            database_report,
            "roseau.local",
            &player_manager,
        );

        assert_eq!(
            report.runtime_plans(),
            &[
                RoseauGameTickRuntimeActionPlan::ResolveConfiguredHost {
                    host: "roseau.local".to_owned(),
                },
                RoseauGameTickRuntimeActionPlan::Network(PlayerNetworkEffect::CloseConnection {
                    connection_id: 21,
                }),
            ]
        );
        assert!(report
            .database_report()
            .database_result()
            .results()
            .is_empty());
    }
}
