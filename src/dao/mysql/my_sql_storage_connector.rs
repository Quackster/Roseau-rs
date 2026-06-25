use crate::dao::mysql::{DatabaseEngine, MySqlDriver, Storage, StorageConnector};
use crate::dao::DaoError;

#[derive(Debug, Default, Clone)]
pub struct MySqlStorageConnector {
    driver: Option<MySqlDriver>,
}

impl MySqlStorageConnector {
    pub fn new() -> Self {
        Self { driver: None }
    }

    pub fn with_driver(driver: MySqlDriver) -> Self {
        Self {
            driver: Some(driver),
        }
    }

    pub fn driver(&self) -> Option<&MySqlDriver> {
        self.driver.as_ref()
    }
}

impl StorageConnector for MySqlStorageConnector {
    fn connect(&self, storage: &Storage) -> Result<(), DaoError> {
        validate_mysql_storage(storage)?;

        let driver = match &self.driver {
            Some(driver) => driver.clone(),
            None => MySqlDriver::connect_storage(storage)?,
        };
        driver
            .pool()
            .get_conn()
            .map(drop)
            .map_err(|error| DaoError::new(format!("MySQL storage connection failed: {error}")))
    }
}

fn validate_mysql_storage(storage: &Storage) -> Result<(), DaoError> {
    if storage.engine() == DatabaseEngine::MySql {
        Ok(())
    } else {
        Err(DaoError::new(format!(
            "MySQL connector cannot validate {} storage",
            storage.engine().config_prefix()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
