use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseEngine {
    MySql,
    Postgres,
    MsSql,
    Sqlite,
}

impl DatabaseEngine {
    pub fn config_prefix(self) -> &'static str {
        match self {
            Self::MySql => "mysql",
            Self::Postgres => "postgres",
            Self::MsSql => "mssql",
            Self::Sqlite => "sqlite",
        }
    }

    pub fn default_port(self) -> u16 {
        match self {
            Self::MySql => 3306,
            Self::Postgres => 5432,
            Self::MsSql => 1433,
            Self::Sqlite => 0,
        }
    }

    pub fn driver_crate(self) -> &'static str {
        match self {
            Self::MySql => "mysql",
            Self::Postgres => "postgres",
            Self::MsSql => "tiberius",
            Self::Sqlite => "rusqlite",
        }
    }

    pub fn is_supported(value: &str) -> bool {
        value.parse::<Self>().is_ok()
    }

    pub fn connection_url(
        self,
        hostname: &str,
        port: u16,
        database: &str,
        sqlite_path: &str,
        options: &str,
    ) -> String {
        match self {
            Self::MySql => append_query(format!("mysql://{hostname}:{port}/{database}"), options),
            Self::Postgres => append_query(
                format!("postgresql://{hostname}:{port}/{database}"),
                options,
            ),
            Self::MsSql => append_mssql_options(
                format!("sqlserver://{hostname}:{port};databaseName={database}"),
                options,
            ),
            Self::Sqlite => format!("sqlite:{sqlite_path}"),
        }
    }
}

impl FromStr for DatabaseEngine {
    type Err = ParseDatabaseEngineError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "mysql" => Ok(Self::MySql),
            "postgres" | "postgresql" => Ok(Self::Postgres),
            "mssql" | "sqlserver" | "sql_server" => Ok(Self::MsSql),
            "sqlite" | "sqlite3" => Ok(Self::Sqlite),
            _ => Err(ParseDatabaseEngineError::new(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDatabaseEngineError {
    value: String,
}

impl ParseDatabaseEngineError {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl Display for ParseDatabaseEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported database engine: {}", self.value)
    }
}

impl std::error::Error for ParseDatabaseEngineError {}

fn append_query(url: String, options: &str) -> String {
    let options = options.trim();
    if options.is_empty() {
        url
    } else {
        format!("{url}?{options}")
    }
}

fn append_mssql_options(url: String, options: &str) -> String {
    let options = options.trim();
    if options.is_empty() {
        url
    } else {
        format!("{url};{options}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
