use crate::dao::mysql::{Storage, StorageConnectionEffect, StorageConnector};
use crate::dao::DaoError;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StdStorageConnector;

impl StdStorageConnector {
    pub fn new() -> Self {
        Self
    }
}

impl StorageConnector for StdStorageConnector {
    fn connect(&self, storage: &Storage) -> Result<(), DaoError> {
        validate_connection_plan(storage.connection_plan())
    }
}

fn validate_connection_plan(
    effects: impl IntoIterator<Item = StorageConnectionEffect>,
) -> Result<(), DaoError> {
    let mut opens_connection = false;

    for effect in effects {
        match effect {
            StorageConnectionEffect::LoadDriverCrate { crate_name } => {
                if crate_name.trim().is_empty() {
                    return Err(DaoError::new("storage driver crate name is empty"));
                }
            }
            StorageConnectionEffect::OpenConnection { connection_url, .. } => {
                opens_connection = true;
                if connection_url.trim().is_empty() {
                    return Err(DaoError::new("storage connection URL is empty"));
                }
            }
        }
    }

    if opens_connection {
        Ok(())
    } else {
        Err(DaoError::new(
            "storage connection plan did not open a connection",
        ))
    }
}

#[cfg(test)]
mod tests {
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
}
