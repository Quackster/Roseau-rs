use std::path::{Path, PathBuf};

use crate::runtime::{
    RoseauApplicationEntrypointSettingsError, RoseauApplicationLoopRunner, RoseauBootstrap,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauApplicationEntrypointSettings {
    main_config_path: PathBuf,
    hotel_config_path: PathBuf,
}

impl RoseauApplicationEntrypointSettings {
    pub fn new(
        main_config_path: impl Into<PathBuf>,
        hotel_config_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            main_config_path: main_config_path.into(),
            hotel_config_path: hotel_config_path.into(),
        }
    }

    pub fn main_config_path(&self) -> &Path {
        &self.main_config_path
    }

    pub fn hotel_config_path(&self) -> &Path {
        &self.hotel_config_path
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

impl Default for RoseauApplicationEntrypointSettings {
    fn default() -> Self {
        Self::new("roseau.properties", "habbohotel.properties")
    }
}

#[cfg(test)]
#[path = "roseau_application_entrypoint_settings_tests.rs"]
mod tests;
