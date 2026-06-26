use crate::game::GameLoadReadiness;
use crate::runtime::RoseauStartupRuntimeStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauApplicationPrepareReadiness {
    database_connected: bool,
    game_load_readiness: Option<GameLoadReadiness>,
    startup_status: Option<RoseauStartupRuntimeStatus>,
}

impl RoseauApplicationPrepareReadiness {
    pub fn new(
        database_connected: bool,
        game_load_readiness: Option<GameLoadReadiness>,
        startup_status: Option<RoseauStartupRuntimeStatus>,
    ) -> Self {
        Self {
            database_connected,
            game_load_readiness,
            startup_status,
        }
    }

    pub fn ready(&self) -> bool {
        self.database_connected
            && self
                .game_load_readiness
                .as_ref()
                .is_some_and(GameLoadReadiness::ready)
            && self
                .startup_status
                .as_ref()
                .is_some_and(RoseauStartupRuntimeStatus::ready)
    }

    pub fn database_connected(&self) -> bool {
        self.database_connected
    }

    pub fn game_load_readiness(&self) -> Option<&GameLoadReadiness> {
        self.game_load_readiness.as_ref()
    }

    pub fn startup_status(&self) -> Option<&RoseauStartupRuntimeStatus> {
        self.startup_status.as_ref()
    }
}

#[cfg(test)]
#[path = "roseau_application_prepare_readiness_tests.rs"]
mod tests;
