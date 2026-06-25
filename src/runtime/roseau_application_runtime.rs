use crate::game::Game;
use crate::runtime::RoseauStartupRuntime;
use crate::runtime::{BootstrapError, RoseauBootstrap, RoseauRuntime, RoseauStartupPlan};
use crate::server::ServerSocketBinder;

pub struct RoseauApplicationRuntime {
    pub(super) runtime: RoseauRuntime,
    pub(super) game: Game,
    pub(super) startup_runtime: RoseauStartupRuntime,
    pub(super) startup_log_lines: Vec<String>,
    pub(super) resolved_config_ip: Option<String>,
}

impl RoseauApplicationRuntime {
    pub fn prepare<B: ServerSocketBinder>(
        bootstrap: &RoseauBootstrap,
        binder: &B,
        public_room_ids: impl IntoIterator<Item = i32>,
        first_connection_id: i32,
        resolved_config_ip: Option<&str>,
    ) -> Result<Self, BootstrapError> {
        let runtime = RoseauRuntime::load(bootstrap)?;
        let mut game = Game::new();
        game.load(runtime.hotel_config())?;
        let server_plan = bootstrap.server_plan(runtime.main_config(), public_room_ids)?;
        let startup_plan = RoseauStartupPlan::from_server_plan(server_plan)?;
        let startup_runtime =
            RoseauStartupRuntime::prepare(&runtime, startup_plan, binder, first_connection_id)?;
        let startup_log_lines = startup_runtime.startup_log_lines(resolved_config_ip);

        Ok(Self {
            runtime,
            game,
            startup_runtime,
            startup_log_lines,
            resolved_config_ip: resolved_config_ip.map(str::to_owned),
        })
    }
}
