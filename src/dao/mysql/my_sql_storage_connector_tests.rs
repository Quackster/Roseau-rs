use super::my_sql_storage_connector::*;
use crate::dao::mysql::DatabaseEngine;

#[test]
fn rejects_non_mysql_storage_urls_before_opening_connection() {
    let storage = Storage::new(
        DatabaseEngine::Postgres,
        "db",
        5432,
        "roseau",
        "secret",
        "hotel",
        "",
        "",
    );

    assert_eq!(
        MySqlStorageConnector::new()
            .connect(&storage)
            .unwrap_err()
            .message(),
        "MySQL connector cannot validate postgres storage"
    );
}

#[test]
fn starts_without_shared_driver() {
    assert!(MySqlStorageConnector::new().driver().is_none());
}
