use super::roseau_application_entrypoint_report::*;
use crate::dao::mysql::{MySqlDaoConnectionReport, MySqlDaoEffect, StorageConnectionOutcome};

#[test]
fn reports_preparation_and_optional_loop_execution() {
    let prepare_report = RoseauApplicationPrepareReport::new(
        MySqlDaoConnectionReport::new(
            StorageConnectionOutcome::Failed {
                message: "database unavailable".to_owned(),
            },
            [MySqlDaoEffect::ConnectStorage],
        ),
        None,
    );

    let report = RoseauApplicationEntrypointReport::new(prepare_report, None);

    assert!(!report.ready());
    assert!(!report.ran_loop());
    assert!(!report.status().ready());
    assert_eq!(report.status().loop_stopped(), None);
    assert!(report.loop_report().is_none());
    assert_eq!(report.log_lines(), Vec::<String>::new());
}

#[test]
fn exposes_database_log_lines() {
    let prepare_report = RoseauApplicationPrepareReport::new(
        MySqlDaoConnectionReport::new(
            StorageConnectionOutcome::Connected,
            [
                MySqlDaoEffect::LogLine("Connecting to mysql database".to_owned()),
                MySqlDaoEffect::ConnectStorage,
                MySqlDaoEffect::LogLine("Connection to mysql was a success".to_owned()),
            ],
        ),
        None,
    );

    let report = RoseauApplicationEntrypointReport::new(prepare_report, None);

    assert_eq!(
        report.log_lines(),
        vec![
            "Connecting to mysql database".to_owned(),
            "Connection to mysql was a success".to_owned(),
        ]
    );
}

#[test]
fn writes_log_lines_through_prepare_report_logger() {
    use crate::logging::Logger;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("roseau-rs-entrypoint-report-{nonce}"));
    let prepare_report = RoseauApplicationPrepareReport::with_logger(
        MySqlDaoConnectionReport::new(
            StorageConnectionOutcome::Connected,
            [
                MySqlDaoEffect::LogLine("Connecting to mysql database".to_owned()),
                MySqlDaoEffect::LogLine("Connection to mysql was a success".to_owned()),
            ],
        ),
        None,
        Logger::new(true, false, &dir),
    );
    let report = RoseauApplicationEntrypointReport::new(prepare_report, None);

    report.write_output_logs().unwrap();

    assert_eq!(
        fs::read_to_string(dir.join("output.log")).unwrap(),
        "Connecting to mysql database\nConnection to mysql was a success\n"
    );
    fs::remove_dir_all(dir).unwrap();
}
