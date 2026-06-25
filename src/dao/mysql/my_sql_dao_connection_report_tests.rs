use super::my_sql_dao_connection_report::*;

#[test]
fn exposes_connection_outcome_and_effects() {
    let report = MySqlDaoConnectionReport::new(
        StorageConnectionOutcome::Failed {
            message: "database unavailable".to_owned(),
        },
        [
            MySqlDaoEffect::ConnectStorage,
            MySqlDaoEffect::LogLine("Could not connect".to_owned()),
        ],
    );

    assert!(!report.connected());
    assert_eq!(report.error(), Some("database unavailable"));
    assert_eq!(
        report.effects(),
        &[
            MySqlDaoEffect::ConnectStorage,
            MySqlDaoEffect::LogLine("Could not connect".to_owned()),
        ]
    );
}
