use std::time::{SystemTime, UNIX_EPOCH};

use super::properties_config::*;

fn temp_file(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("roseau-rs-{name}-{nonce}.properties"))
}

#[test]
fn parses_java_properties_values() {
    let config = PropertiesConfig::parse(
        r#"
        # comment
        server.port=37120
        log.output:true
        blank
        "#,
    );

    assert_eq!(config.get("server.port"), Some("37120"));
    assert_eq!(config.get_integer("server.port").unwrap(), 37120);
    assert!(config.get_boolean("log.output"));
    assert_eq!(config.get("blank"), None);
}

#[test]
fn load_tracks_source_file() {
    let path = temp_file("properties");
    fs::write(&path, "server.port=30000").unwrap();

    let config = PropertiesConfig::load(&path).unwrap();

    assert_eq!(config.file(), Some(path.as_path()));
    assert_eq!(config.get_integer("server.port").unwrap(), 30000);

    fs::remove_file(path).unwrap();
}
