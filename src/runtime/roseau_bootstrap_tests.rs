use std::time::{SystemTime, UNIX_EPOCH};

use super::*;

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("roseau-rs-bootstrap-{name}-{nonce}"))
}

#[test]
fn creates_default_config_files_once() {
    let root = temp_dir("config");
    let bootstrap = RoseauBootstrap::new(
        root.join("roseau.properties"),
        root.join("habbohotel.properties"),
    );

    assert_eq!(bootstrap.ensure_config_files().unwrap(), [true, true]);
    assert_eq!(bootstrap.ensure_config_files().unwrap(), [false, false]);
    assert!(bootstrap.load_main_config().is_ok());
    assert!(bootstrap.load_hotel_config().is_ok());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn builds_server_plan_from_config_and_public_rooms() {
    let config = Config::parse(DEFAULT_MAIN_CONFIG).unwrap();
    let bootstrap = RoseauBootstrap::default_paths();

    let plan = bootstrap.server_plan(&config, [5, 7]).unwrap();

    assert_eq!(plan.bind_ip(), "127.0.0.1");
    assert_eq!(plan.raw_config_ip(), "127.0.0.1");
    assert_eq!(plan.server_port(), 37120);
    assert_eq!(plan.private_server_port(), 37119);
    assert_eq!(plan.ports(), &[37120, 37119, 37125, 37127]);
    assert_eq!(plan.database_engine(), DatabaseEngine::MySql);
}

#[test]
fn hostname_configs_bind_to_wildcard_until_resolved_by_runtime() {
    let config = Config::parse(
        "[Server]\nserver.ip=localhost\nserver.port=37120\nserver.private.port=37119\nserver.class.path=roseau::server::ServerHandler\n\n[Database]\ntype=mysql\n",
    )
    .unwrap();
    let bootstrap = RoseauBootstrap::default_paths();

    let plan = bootstrap.server_plan(&config, []).unwrap();

    assert_eq!(plan.bind_ip(), "0.0.0.0");
    assert_eq!(plan.raw_config_ip(), "localhost");
}
