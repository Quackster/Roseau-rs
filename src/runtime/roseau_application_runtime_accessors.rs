use crate::game::Game;
use crate::runtime::{
    RoseauApplicationRuntime, RoseauRuntime, RoseauStartupRuntime, RoseauStartupRuntimeStatus,
};

impl RoseauApplicationRuntime {
    pub fn runtime(&self) -> &RoseauRuntime {
        &self.runtime
    }

    pub fn game(&self) -> &Game {
        &self.game
    }

    pub fn game_mut(&mut self) -> &mut Game {
        &mut self.game
    }

    pub fn resolved_config_ip(&self) -> Option<&str> {
        self.resolved_config_ip.as_deref()
    }

    pub(crate) fn set_resolved_config_ip(&mut self, address: String) {
        self.resolved_config_ip = Some(address);
    }

    pub fn advertised_server_ip(&self) -> String {
        self.startup_runtime
            .startup_plan()
            .server_plan()
            .advertised_ip(self.resolved_config_ip())
    }

    pub fn startup_runtime(&self) -> &RoseauStartupRuntime {
        &self.startup_runtime
    }

    pub fn startup_runtime_mut(&mut self) -> &mut RoseauStartupRuntime {
        &mut self.startup_runtime
    }

    pub fn startup_log_lines(&self) -> &[String] {
        &self.startup_log_lines
    }

    pub fn status(&self) -> RoseauStartupRuntimeStatus {
        self.startup_runtime.status()
    }
}
