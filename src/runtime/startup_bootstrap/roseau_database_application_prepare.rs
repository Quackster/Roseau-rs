use crate::dao::mysql::{MySqlDao, StorageConnector};
use crate::dao::PublicRoomDescriptor;
use crate::game::Game;
use crate::runtime::{
    BootstrapError, RoseauApplicationPrepareReport, RoseauApplicationRuntime, RoseauBootstrap,
    RoseauRuntime, RoseauStartupPlan, RoseauStartupRuntime,
};
use crate::server::ServerSocketBinder;

impl RoseauApplicationRuntime {
    pub fn prepare_with_database_connector<B: ServerSocketBinder, C: StorageConnector>(
        bootstrap: &RoseauBootstrap,
        binder: &B,
        connector: &C,
        public_rooms: impl IntoIterator<Item = PublicRoomDescriptor>,
        first_connection_id: i32,
        resolved_config_ip: Option<&str>,
    ) -> Result<RoseauApplicationPrepareReport, BootstrapError> {
        let runtime = RoseauRuntime::load(bootstrap)?;
        let logger = runtime.logger().clone();
        let mut dao = MySqlDao::from_config(runtime.main_config())?;
        let database_report = dao.connect_report_with(connector);

        if !database_report.connected() {
            return Ok(RoseauApplicationPrepareReport::with_logger(
                database_report,
                None,
                logger,
            ));
        }

        let mut game = Game::new();
        game.load(runtime.hotel_config())?;
        let server_plan = bootstrap.server_plan(runtime.main_config(), public_rooms)?;
        let startup_plan = RoseauStartupPlan::from_server_plan(server_plan)?;
        let startup_runtime =
            RoseauStartupRuntime::prepare(&runtime, startup_plan, binder, first_connection_id)?;
        let startup_log_lines = startup_runtime.startup_log_lines(resolved_config_ip);
        let application_runtime = Self {
            runtime,
            game,
            startup_runtime,
            startup_log_lines,
            resolved_config_ip: resolved_config_ip.map(str::to_owned),
        };

        Ok(RoseauApplicationPrepareReport::with_logger(
            database_report,
            Some(application_runtime),
            logger,
        ))
    }
}
