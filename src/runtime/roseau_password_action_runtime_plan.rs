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
mod tests {
    use super::*;
    use crate::dao::mysql::{MySqlPlayerPasswordActionReport, SqlExecutionBatchResult};
    use crate::game::player::{
        PlayerDetails, PlayerLoginOutcome, PlayerPasswordActionOutcome, PlayerRegistrationOutcome,
        PlayerSession,
    };

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    fn execution_report(
        outcomes: impl Into<Vec<PlayerPasswordActionOutcome>>,
    ) -> MySqlPlayerPasswordActionExecutionReport {
        MySqlPlayerPasswordActionExecutionReport::new(
            MySqlPlayerPasswordActionReport::from_outcomes(outcomes, 11, 1234),
            SqlExecutionBatchResult::new([]),
        )
    }

    #[test]
    fn collects_direct_password_network_effects() {
        let player_manager = PlayerManager::new(Vec::new());
        let report = execution_report([PlayerPasswordActionOutcome::Login(
            PlayerLoginOutcome::failed(),
        )]);

        assert_eq!(
            RoseauPasswordActionRuntimePlan::collect(&report, &player_manager),
            vec![RoseauPasswordActionRuntimePlan::Network(
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 11,
                    packet: "#ERROR Login incorrect##".to_owned(),
                }
            )]
        );
    }

    #[test]
    fn collects_registration_packet_and_duplicate_login_close() {
        let mut player_manager = PlayerManager::new(Vec::new());
        player_manager.insert(PlayerSession::new(42, 30001, details(7, "alice-old")));
        let report = execution_report([
            PlayerPasswordActionOutcome::Login(PlayerLoginOutcome::authenticated(
                &details(7, "alice"),
                "secret",
                false,
                30001,
                30001,
                Some(42),
            )),
            PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::Created),
        ]);

        assert_eq!(
            RoseauPasswordActionRuntimePlan::collect(&report, &player_manager),
            vec![
                RoseauPasswordActionRuntimePlan::Network(PlayerNetworkEffect::WriteResponse {
                    connection_id: 11,
                    packet: "#OK##".to_owned(),
                }),
                RoseauPasswordActionRuntimePlan::Network(PlayerNetworkEffect::CloseConnection {
                    connection_id: 42,
                }),
            ]
        );
    }

    #[test]
    fn leaves_persistence_only_login_effects_out_of_runtime_network_plans() {
        let player_manager = PlayerManager::new(Vec::new());
        let report = execution_report([PlayerPasswordActionOutcome::Login(
            PlayerLoginOutcome::authenticated(
                &details(7, "alice"),
                "secret",
                false,
                30001,
                30001,
                None,
            ),
        )]);

        assert!(RoseauPasswordActionRuntimePlan::collect(&report, &player_manager).is_empty());
    }
}
