use super::config::*;

#[test]
fn parses_ini_sections_and_typed_values() {
    let config = Config::parse(
        r#"
        [Server]
        server.ip=127.0.0.1
        server.port=37120

        [Logging]
        log.packets=true
        "#,
    )
    .unwrap();

    assert_eq!(config.get("Server", "server.ip"), Some("127.0.0.1"));
    assert_eq!(
        config.parse_value::<u16>("Server", "server.port").unwrap(),
        37120
    );
    assert!(config.get_bool("Logging", "log.packets").unwrap());
}

#[test]
fn rejects_malformed_lines() {
    let error = Config::parse("[Server]\nserver.port").unwrap_err();
    assert!(matches!(error, ConfigError::InvalidLine { line: 2, .. }));
}
