use crate::dao::mysql::Storage;
use crate::dao::DaoError;

pub trait StorageConnector {
    fn connect(&self, storage: &Storage) -> Result<(), DaoError>;
}
