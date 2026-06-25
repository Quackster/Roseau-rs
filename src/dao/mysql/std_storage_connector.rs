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
