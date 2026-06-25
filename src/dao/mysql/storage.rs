use crate::config::Config;
use crate::dao::DaoError;

use super::{DatabaseEngine, SqlExecutionPlan, SqlQuery, StorageConnectionEffect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Storage {
    engine: DatabaseEngine,
    connection_url: String,
    username: String,
    password: String,
}

impl Storage {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        engine: DatabaseEngine,
        host: impl AsRef<str>,
        port: u16,
        username: impl Into<String>,
        password: impl Into<String>,
        database: impl AsRef<str>,
        sqlite_path: impl AsRef<str>,
        options: impl AsRef<str>,
    ) -> Self {
        let connection_url = engine.connection_url(
            host.as_ref(),
            port,
            database.as_ref(),
            sqlite_path.as_ref(),
            options.as_ref(),
        );

        Self {
            engine,
            connection_url,
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn from_config(config: &Config) -> Result<Self, DaoError> {
        let engine = database_setting(config, None, "type", "mysql")
            .parse::<DatabaseEngine>()
            .map_err(|error| DaoError::new(error.to_string()))?;
        let prefix = engine.config_prefix();
        let port = database_setting(config, Some(prefix), "port", "")
            .parse::<u16>()
            .unwrap_or_else(|_| engine.default_port());

        Ok(Self::new(
            engine,
            database_setting(config, Some(prefix), "hostname", "127.0.0.1"),
            port,
            database_setting(config, Some(prefix), "username", ""),
            database_setting(config, Some(prefix), "password", ""),
            database_setting(config, Some(prefix), "database", "roseau"),
            database_setting(config, Some(prefix), "path", "roseau.sqlite"),
            database_setting(config, Some(prefix), "options", ""),
        ))
    }

    pub fn engine(&self) -> DatabaseEngine {
        self.engine
    }

    pub fn connection_url(&self) -> &str {
        &self.connection_url
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn uses_credentials(&self) -> bool {
        !self.username.trim().is_empty()
    }

    pub fn connection_plan(&self) -> Vec<StorageConnectionEffect> {
        vec![
            StorageConnectionEffect::LoadDriverCrate {
                crate_name: self.engine.driver_crate(),
            },
            StorageConnectionEffect::OpenConnection {
                connection_url: self.connection_url.clone(),
                username: self.uses_credentials().then(|| self.username.clone()),
            },
        ]
    }

    pub fn context_read_plan(&self, query: SqlQuery) -> SqlExecutionPlan {
        query.read_plan()
    }

    pub fn context_execute_plan(&self, query: SqlQuery) -> SqlExecutionPlan {
        query.execute_plan()
    }

    pub fn context_insert_returning_id_plan(&self, query: SqlQuery) -> SqlExecutionPlan {
        query.insert_returning_id_plan()
    }
}

fn database_setting(config: &Config, prefix: Option<&str>, key: &str, default: &str) -> String {
    if let Some(value) = config
        .get("Database", key)
        .filter(|value| !value.trim().is_empty())
    {
        return value.trim().to_owned();
    }

    if let Some(prefix) = prefix {
        let prefixed_key = format!("{prefix}.{key}");
        if let Some(value) = config
            .get("Database", &prefixed_key)
            .filter(|value| !value.trim().is_empty())
        {
            return value.trim().to_owned();
        }
    }

    default.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
