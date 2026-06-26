use std::fmt::{self, Display};
use std::io;

use crate::config::ConfigError;
use crate::dao::DaoError;

#[derive(Debug)]
pub enum BootstrapError {
    Config(ConfigError),
    Dao(DaoError),
    Io(io::Error),
    PortOutOfRange { key: &'static str, value: i32 },
    RoomPortOutOfRange { base_port: u16, room_id: i32 },
    UnsupportedServerHandler { class_path: String },
}

impl From<ConfigError> for BootstrapError {
    fn from(error: ConfigError) -> Self {
        Self::Config(error)
    }
}

impl From<DaoError> for BootstrapError {
    fn from(error: DaoError) -> Self {
        Self::Dao(error)
    }
}

impl From<io::Error> for BootstrapError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl Display for BootstrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(error) => write!(f, "{error}"),
            Self::Dao(error) => write!(f, "{error}"),
            Self::Io(error) => write!(f, "{error}"),
            Self::PortOutOfRange { key, value } => {
                write!(f, "config key {key} has invalid port value {value}")
            }
            Self::RoomPortOutOfRange { base_port, room_id } => {
                write!(
                    f,
                    "room id {room_id} cannot be added to base port {base_port}"
                )
            }
            Self::UnsupportedServerHandler { class_path } => {
                write!(f, "unsupported server handler: {class_path}")
            }
        }
    }
}

impl std::error::Error for BootstrapError {}
