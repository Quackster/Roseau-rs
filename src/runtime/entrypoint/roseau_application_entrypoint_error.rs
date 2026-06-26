use crate::dao::DaoError;
use crate::runtime::BootstrapError;

#[derive(Debug)]
pub enum RoseauApplicationEntrypointError {
    Bootstrap(BootstrapError),
    Database(DaoError),
}

impl From<BootstrapError> for RoseauApplicationEntrypointError {
    fn from(error: BootstrapError) -> Self {
        Self::Bootstrap(error)
    }
}

impl From<DaoError> for RoseauApplicationEntrypointError {
    fn from(error: DaoError) -> Self {
        Self::Database(error)
    }
}
