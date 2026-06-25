use super::*;

#[test]
fn defaults_match_rust_entrypoint_startup_parameters() {
    let settings = RoseauApplicationEntrypointSettings::default();

    assert_eq!(settings.main_config_path(), Path::new("roseau.properties"));
    assert_eq!(
        settings.hotel_config_path(),
        Path::new("habbohotel.properties")
    );
    assert_eq!(settings.first_connection_id(), 1);
    assert_eq!(settings.listener_index(), 0);
    assert!(settings.accept_connection());
    assert_eq!(settings.max_bytes(), 4096);
    assert_eq!(settings.loop_runner().max_ticks(), None);
    assert_eq!(
        settings.bootstrap().main_config_path(),
        Path::new("roseau.properties")
    );
}

#[test]
fn parses_supported_entrypoint_arguments() {
    let settings = RoseauApplicationEntrypointSettings::from_args([
        "--main-config".to_owned(),
        "custom.properties".to_owned(),
        "--hotel-config".to_owned(),
        "hotel.properties".to_owned(),
        "--first-connection-id".to_owned(),
        "10".to_owned(),
        "--listener-index".to_owned(),
        "2".to_owned(),
        "--no-accept-connection".to_owned(),
        "--max-bytes".to_owned(),
        "8192".to_owned(),
    ])
    .unwrap();

    assert_eq!(settings.main_config_path(), Path::new("custom.properties"));
    assert_eq!(settings.hotel_config_path(), Path::new("hotel.properties"));
    assert_eq!(settings.first_connection_id(), 10);
    assert_eq!(settings.listener_index(), 2);
    assert!(!settings.accept_connection());
    assert_eq!(settings.max_bytes(), 8192);
}

#[test]
fn rejects_unknown_or_invalid_entrypoint_arguments() {
    assert_eq!(
        RoseauApplicationEntrypointSettings::from_args(["--bad".to_owned()])
            .unwrap_err()
            .message(),
        "unknown argument --bad"
    );
    assert_eq!(
        RoseauApplicationEntrypointSettings::from_args(["--max-ticks".to_owned()])
            .unwrap_err()
            .message(),
        "unknown argument --max-ticks"
    );
}
