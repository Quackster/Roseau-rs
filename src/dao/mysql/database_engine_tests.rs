use super::database_engine::*;

#[test]
fn parses_java_database_engine_aliases() {
    assert_eq!(
        "mysql".parse::<DatabaseEngine>().unwrap(),
        DatabaseEngine::MySql
    );
    assert_eq!(
        "postgresql".parse::<DatabaseEngine>().unwrap(),
        DatabaseEngine::Postgres
    );
    assert_eq!(
        "sql_server".parse::<DatabaseEngine>().unwrap(),
        DatabaseEngine::MsSql
    );
    assert_eq!(
        "sqlite3".parse::<DatabaseEngine>().unwrap(),
        DatabaseEngine::Sqlite
    );
    assert!(!DatabaseEngine::is_supported("oracle"));
}

#[test]
fn maps_engines_to_rust_driver_crates() {
    assert_eq!(DatabaseEngine::MySql.driver_crate(), "mysql");
    assert_eq!(DatabaseEngine::Postgres.driver_crate(), "postgres");
    assert_eq!(DatabaseEngine::MsSql.driver_crate(), "tiberius");
    assert_eq!(DatabaseEngine::Sqlite.driver_crate(), "rusqlite");
}

#[test]
fn builds_rust_connection_urls() {
    assert_eq!(
        DatabaseEngine::MySql.connection_url(
            "localhost",
            3307,
            "roseau",
            "roseau.sqlite",
            "ssl=false",
        ),
        "mysql://localhost:3307/roseau?ssl=false"
    );
    assert_eq!(
        DatabaseEngine::MsSql.connection_url("db", 1433, "roseau", "", "encrypt=false"),
        "sqlserver://db:1433;databaseName=roseau;encrypt=false"
    );
    assert_eq!(
        DatabaseEngine::Sqlite.connection_url("", 0, "", "roseau.sqlite", ""),
        "sqlite:roseau.sqlite"
    );
}
