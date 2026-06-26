use super::*;

#[test]
fn defaults_match_rust_entrypoint_startup_parameters() {
    let settings = RoseauApplicationEntrypointSettings::default();

    assert_eq!(settings.main_config_path(), Path::new("roseau.properties"));
    assert_eq!(
        settings.hotel_config_path(),
        Path::new("habbohotel.properties")
    );
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
    ])
    .unwrap();

    assert_eq!(settings.main_config_path(), Path::new("custom.properties"));
    assert_eq!(settings.hotel_config_path(), Path::new("hotel.properties"));
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
