use std::path::{Path, PathBuf};

use crate::runtime::{
    RoseauApplicationEntrypointSettingsError, RoseauApplicationLoopRunner, RoseauBootstrap,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauApplicationEntrypointSettings {
    main_config_path: PathBuf,
    hotel_config_path: PathBuf,
    first_connection_id: i32,
    listener_index: usize,
    accept_connection: bool,
    max_bytes: usize,
}

impl RoseauApplicationEntrypointSettings {
    pub fn new(
        main_config_path: impl Into<PathBuf>,
        hotel_config_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            main_config_path: main_config_path.into(),
            hotel_config_path: hotel_config_path.into(),
            first_connection_id: 1,
            listener_index: 0,
            accept_connection: true,
            max_bytes: 4096,
        }
    }

    pub fn main_config_path(&self) -> &Path {
        &self.main_config_path
    }

    pub fn hotel_config_path(&self) -> &Path {
        &self.hotel_config_path
    }

    pub fn first_connection_id(&self) -> i32 {
        self.first_connection_id
    }

    pub fn listener_index(&self) -> usize {
        self.listener_index
    }

    pub fn accept_connection(&self) -> bool {
        self.accept_connection
    }

    pub fn max_bytes(&self) -> usize {
        self.max_bytes
    }

    pub fn with_first_connection_id(mut self, first_connection_id: i32) -> Self {
        self.first_connection_id = first_connection_id;
        self
    }

    pub fn with_listener_index(mut self, listener_index: usize) -> Self {
        self.listener_index = listener_index;
        self
    }

    pub fn with_accept_connection(mut self, accept_connection: bool) -> Self {
        self.accept_connection = accept_connection;
        self
    }

    pub fn with_max_bytes(mut self, max_bytes: usize) -> Self {
        self.max_bytes = max_bytes;
        self
    }

    pub fn from_args(
        args: impl IntoIterator<Item = String>,
    ) -> Result<Self, RoseauApplicationEntrypointSettingsError> {
        let mut settings = Self::default();
        let mut args = args.into_iter();

        while let Some(argument) = args.next() {
            match argument.as_str() {
                "--main-config" => {
                    settings.main_config_path = required_arg(&mut args, "--main-config")?.into();
                }
                "--hotel-config" => {
                    settings.hotel_config_path = required_arg(&mut args, "--hotel-config")?.into();
                }
                "--first-connection-id" => {
                    settings.first_connection_id = parse_arg(&mut args, "--first-connection-id")?;
                }
                "--listener-index" => {
                    settings.listener_index = parse_arg(&mut args, "--listener-index")?;
                }
                "--max-bytes" => {
                    settings.max_bytes = parse_arg(&mut args, "--max-bytes")?;
                }
                "--accept-connection" => {
                    settings.accept_connection = true;
                }
                "--no-accept-connection" => {
                    settings.accept_connection = false;
                }
                _ => {
                    return Err(RoseauApplicationEntrypointSettingsError::new(format!(
                        "unknown argument {argument}"
                    )));
                }
            }
        }

        Ok(settings)
    }

    pub fn bootstrap(&self) -> RoseauBootstrap {
        RoseauBootstrap::new(
            self.main_config_path.clone(),
            self.hotel_config_path.clone(),
        )
    }

    pub fn loop_runner(&self) -> RoseauApplicationLoopRunner {
        RoseauApplicationLoopRunner::new()
    }
}

fn required_arg(
    args: &mut impl Iterator<Item = String>,
    name: &str,
) -> Result<String, RoseauApplicationEntrypointSettingsError> {
    args.next().ok_or_else(|| {
        RoseauApplicationEntrypointSettingsError::new(format!("missing value for {name}"))
    })
}

fn parse_arg<T: std::str::FromStr>(
    args: &mut impl Iterator<Item = String>,
    name: &str,
) -> Result<T, RoseauApplicationEntrypointSettingsError> {
    required_arg(args, name)?.parse::<T>().map_err(|_| {
        RoseauApplicationEntrypointSettingsError::new(format!("invalid value for {name}"))
    })
}

impl Default for RoseauApplicationEntrypointSettings {
    fn default() -> Self {
        Self::new("roseau.properties", "habbohotel.properties")
    }
}

#[cfg(test)]
#[path = "roseau_application_entrypoint_settings_tests.rs"]
mod tests;
