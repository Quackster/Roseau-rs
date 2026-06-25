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
