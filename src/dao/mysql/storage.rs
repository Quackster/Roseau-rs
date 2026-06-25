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
