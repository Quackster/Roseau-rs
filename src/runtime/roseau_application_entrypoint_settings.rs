use std::path::{Path, PathBuf};

use crate::runtime::{
    RoseauApplicationEntrypointSettingsError, RoseauApplicationLoopRunner, RoseauBootstrap,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauApplicationEntrypointSettings {
    main_config_path: PathBuf,
    hotel_config_path: PathBuf,
    max_ticks: usize,
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
            max_ticks: 1,
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

    pub fn max_ticks(&self) -> usize {
        self.max_ticks
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

    pub fn with_max_ticks(mut self, max_ticks: usize) -> Self {
        self.max_ticks = max_ticks;
        self
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
                "--max-ticks" => {
                    settings.max_ticks = parse_arg(&mut args, "--max-ticks")?;
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
        RoseauApplicationLoopRunner::new(self.max_ticks)
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
mod tests {
    use super::*;

    #[test]
    fn defaults_match_rust_entrypoint_startup_parameters() {
        let settings = RoseauApplicationEntrypointSettings::default();

        assert_eq!(settings.main_config_path(), Path::new("roseau.properties"));
        assert_eq!(
            settings.hotel_config_path(),
            Path::new("habbohotel.properties")
        );
        assert_eq!(settings.max_ticks(), 1);
        assert_eq!(settings.first_connection_id(), 1);
        assert_eq!(settings.listener_index(), 0);
        assert!(settings.accept_connection());
        assert_eq!(settings.max_bytes(), 4096);
        assert_eq!(settings.loop_runner().max_ticks(), 1);
        assert_eq!(
            settings.bootstrap().main_config_path(),
            Path::new("roseau.properties")
        );
    }

    #[test]
    fn parses_supported_entrypoint_arguments() {
        let settings = RoseauApplicationEntrypointSettings::from_args([
            "--main-config".to_owned(),
            "custom.properties".to_owned(),
            "--hotel-config".to_owned(),
            "hotel.properties".to_owned(),
            "--max-ticks".to_owned(),
            "5".to_owned(),
            "--first-connection-id".to_owned(),
            "10".to_owned(),
            "--listener-index".to_owned(),
            "2".to_owned(),
            "--no-accept-connection".to_owned(),
            "--max-bytes".to_owned(),
            "8192".to_owned(),
        ])
        .unwrap();

        assert_eq!(settings.main_config_path(), Path::new("custom.properties"));
        assert_eq!(settings.hotel_config_path(), Path::new("hotel.properties"));
        assert_eq!(settings.max_ticks(), 5);
        assert_eq!(settings.first_connection_id(), 10);
        assert_eq!(settings.listener_index(), 2);
        assert!(!settings.accept_connection());
        assert_eq!(settings.max_bytes(), 8192);
    }

    #[test]
    fn rejects_unknown_or_invalid_entrypoint_arguments() {
        assert_eq!(
            RoseauApplicationEntrypointSettings::from_args(["--bad".to_owned()])
                .unwrap_err()
                .message(),
            "unknown argument --bad"
        );
        assert_eq!(
            RoseauApplicationEntrypointSettings::from_args([
                "--max-ticks".to_owned(),
                "many".to_owned()
            ])
            .unwrap_err()
            .message(),
            "invalid value for --max-ticks"
        );
    }
}
