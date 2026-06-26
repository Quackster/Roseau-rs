use super::*;
use crate::dao::mysql::DatabaseEngine;

#[test]
fn accepts_non_empty_storage_url() {
    let storage = Storage::new(
        DatabaseEngine::MySql,
        "localhost",
        3306,
        "user",
        "",
        "roseau",
        "",
        "",
    );

    assert!(StdStorageConnector::new().connect(&storage).is_ok());
}

#[test]
fn validates_storage_connection_plan_before_marking_connected() {
    assert_eq!(
        validate_connection_plan([StorageConnectionEffect::OpenConnection {
            connection_url: " ".to_owned(),
            username: None,
        }])
        .unwrap_err()
        .message(),
        "storage connection URL is empty"
    );
    assert_eq!(
        validate_connection_plan([StorageConnectionEffect::LoadDriverCrate { crate_name: "" }])
            .unwrap_err()
            .message(),
        "storage driver crate name is empty"
    );
    assert_eq!(
        validate_connection_plan([]).unwrap_err().message(),
        "storage connection plan did not open a connection"
    );
}
