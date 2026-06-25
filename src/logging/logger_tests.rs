use std::time::{SystemTime, UNIX_EPOCH};

use super::*;

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("roseau-rs-logger-{name}-{nonce}"))
}

#[test]
fn formats_prefixed_lines_and_startup_banner() {
    assert_eq!(
        Logger::line("hello", 0),
        "[01-01-1970 12:01:00] [ROSEAU] >> hello"
    );
    assert_eq!(Logger::empty_line(0), "[01-01-1970 12:01:00] [ROSEAU] ");

    let lines = Logger::startup_lines(0);
    assert_eq!(lines[2], "-- SERVER BOOT TIME: 01-01-1970 12:01:00");
    assert_eq!(
        lines[5],
        "[01-01-1970 12:01:00] [ROSEAU] >> Roseau - Rust Server"
    );
}

#[test]
fn reads_logging_flags_from_config() {
    let config = Config::parse(
        r#"
        [Logging]
        log.output=true
        log.errors=false
        "#,
    )
    .unwrap();

    let logger = Logger::from_config(&config, "log");

    assert!(logger.output_enabled());
    assert!(!logger.error_enabled());
}

#[test]
fn appends_output_and_error_logs_when_enabled() {
    let dir = temp_dir("enabled");
    let logger = Logger::new(true, true, &dir);

    logger.write_output_line("line").unwrap();
    logger.write_error("stack", 0).unwrap();

    assert_eq!(
        fs::read_to_string(dir.join("output.log")).unwrap(),
        "line\n"
    );
    let error_log = fs::read_to_string(dir.join("error.log")).unwrap();
    assert!(error_log.contains("01-01-1970 12:01:00 - Error has occured!"));
    assert!(error_log.contains("stack"));

    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn writes_exception_to_enabled_output_and_error_logs() {
    let dir = temp_dir("exception-enabled");
    let logger = Logger::new(true, true, &dir);

    logger.write_exception("stack", 0).unwrap();

    assert_eq!(
        fs::read_to_string(dir.join("output.log")).unwrap(),
        concat!(
            "[01-01-1970 12:01:00] [ROSEAU] >> ---------------------------------------------\n",
            "[01-01-1970 12:01:00] [ROSEAU] >> Error has occured!\n",
            "stack\n",
            "[01-01-1970 12:01:00] [ROSEAU] >> ---------------------------------------------\n",
        )
    );
    let error_log = fs::read_to_string(dir.join("error.log")).unwrap();
    assert!(error_log.contains("01-01-1970 12:01:00 - Error has occured!"));
    assert!(error_log.contains("stack"));

    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn formats_exception_lines_like_java_logger() {
    assert_eq!(
        Logger::exception_lines("stack trace", 0),
        vec![
            "[01-01-1970 12:01:00] [ROSEAU] >> ---------------------------------------------",
            "[01-01-1970 12:01:00] [ROSEAU] >> Error has occured!",
            "stack trace",
            "[01-01-1970 12:01:00] [ROSEAU] >> ---------------------------------------------",
        ]
    );
}

#[test]
fn leaves_files_absent_when_disabled() {
    let dir = temp_dir("disabled");
    let logger = Logger::new(false, false, &dir);

    logger.write_output_line("line").unwrap();
    logger.write_error("stack", 0).unwrap();
    logger.write_exception("stack", 0).unwrap();

    assert!(!dir.exists());
}
