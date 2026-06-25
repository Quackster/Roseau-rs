use super::*;
use crate::dao::mysql::{MySqlDaoEffect, StorageConnectionOutcome};

#[test]
fn reports_database_and_application_readiness() {
    let database_report = MySqlDaoConnectionReport::new(
        StorageConnectionOutcome::Connected,
        [MySqlDaoEffect::ConnectStorage],
    );
    let report = RoseauApplicationPrepareReport::new(database_report, None);

    assert!(report.database_report().connected());
    assert!(!report.ready());
    assert!(!report.readiness().ready());
    assert!(report.readiness().game_load_readiness().is_none());
    assert!(report.readiness().startup_status().is_none());
    assert!(report.application_runtime().is_none());
    assert!(!report.logger().output_enabled());
}
