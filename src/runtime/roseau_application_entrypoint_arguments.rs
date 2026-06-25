use crate::runtime::{
    RoseauApplicationEntrypointSettings, RoseauApplicationEntrypointSettingsError,
    RoseauApplicationEntrypointUsage,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoseauApplicationEntrypointArguments {
    Run(RoseauApplicationEntrypointSettings),
    Usage,
    Error(RoseauApplicationEntrypointSettingsError),
}

impl RoseauApplicationEntrypointArguments {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Self {
        let args = args.into_iter().collect::<Vec<_>>();

        if RoseauApplicationEntrypointUsage::requested(&args) {
            return Self::Usage;
        }

        match RoseauApplicationEntrypointSettings::from_args(args) {
            Ok(settings) => Self::Run(settings),
            Err(error) => Self::Error(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn routes_help_requests_to_usage() {
        assert_eq!(
            RoseauApplicationEntrypointArguments::parse(["--help".to_owned()]),
            RoseauApplicationEntrypointArguments::Usage
        );
    }

    #[test]
    fn routes_valid_arguments_to_run_settings() {
        let parsed = RoseauApplicationEntrypointArguments::parse([
            "--main-config".to_owned(),
            "custom.properties".to_owned(),
            "--max-ticks".to_owned(),
            "3".to_owned(),
        ]);

        let RoseauApplicationEntrypointArguments::Run(settings) = parsed else {
            panic!("expected run settings");
        };
        assert_eq!(settings.main_config_path(), Path::new("custom.properties"));
        assert_eq!(settings.max_ticks(), 3);
    }

    #[test]
    fn routes_invalid_arguments_to_error() {
        let parsed = RoseauApplicationEntrypointArguments::parse(["--bad".to_owned()]);

        let RoseauApplicationEntrypointArguments::Error(error) = parsed else {
            panic!("expected settings error");
        };
        assert_eq!(error.message(), "unknown argument --bad");
    }
}
