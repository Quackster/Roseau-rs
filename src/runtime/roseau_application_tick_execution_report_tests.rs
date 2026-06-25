use super::roseau_application_tick_execution_report::*;
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
