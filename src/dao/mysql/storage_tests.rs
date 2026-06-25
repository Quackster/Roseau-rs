use super::storage::*;

#[test]
fn builds_storage_from_generic_database_config() {
    let config = Config::parse(
        r#"
        [Database]
        type=mysql
        hostname=db
        port=3307
        username=roseau
        password=secret
        database=hotel
        options=ssl=false
        "#,
    )
    .unwrap();

    let storage = Storage::from_config(&config).unwrap();

    assert_eq!(storage.engine(), DatabaseEngine::MySql);
    assert_eq!(storage.connection_url(), "mysql://db:3307/hotel?ssl=false");
    assert_eq!(storage.username(), "roseau");
    assert!(storage.uses_credentials());
}

#[test]
fn prefixed_database_config_falls_back_like_java() {
    let config = Config::parse(
        r#"
        [Database]
        type=postgresql
        postgres.hostname=pg
        postgres.database=roseau_pg
        "#,
    )
    .unwrap();

    let storage = Storage::from_config(&config).unwrap();

    assert_eq!(storage.engine(), DatabaseEngine::Postgres);
    assert_eq!(storage.connection_url(), "postgresql://pg:5432/roseau_pg");
}

#[test]
fn plans_driver_load_and_connection_attempts() {
    let with_credentials = Storage::new(
        DatabaseEngine::MySql,
        "db",
        3306,
        "roseau",
        "secret",
        "hotel",
        "",
        "",
    );
    let without_credentials = Storage::new(
        DatabaseEngine::Sqlite,
        "",
        0,
        "",
        "",
        "",
        "roseau.sqlite",
        "",
    );

    assert_eq!(
        with_credentials.connection_plan(),
        vec![
            StorageConnectionEffect::LoadDriverCrate {
                crate_name: "mysql",
            },
            StorageConnectionEffect::OpenConnection {
                connection_url: "mysql://db:3306/hotel".to_owned(),
                username: Some("roseau".to_owned()),
            },
        ]
    );
    assert_eq!(
        without_credentials.connection_plan(),
        vec![
            StorageConnectionEffect::LoadDriverCrate {
                crate_name: "rusqlite",
            },
            StorageConnectionEffect::OpenConnection {
                connection_url: "sqlite:roseau.sqlite".to_owned(),
                username: None,
            },
        ]
    );
}

#[test]
fn builds_context_execution_plans_for_queries() {
    let storage = Storage::new(DatabaseEngine::MySql, "db", 3306, "", "", "hotel", "", "");
    let query = SqlQuery::new(
        "DELETE FROM rooms WHERE id = ?",
        [crate::dao::mysql::SqlParameter::Integer(7)],
    );

    let plan = storage.context_execute_plan(query);

    assert_eq!(plan.sql(), "DELETE FROM rooms WHERE id = ?");
    assert_eq!(plan.kind(), crate::dao::mysql::SqlExecutionKind::Execute);
}
