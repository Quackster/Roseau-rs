use crate::dao::mysql::{MySqlApplicationTickExecutor, SqlExecutor, StorageConnector};
use crate::dao::PublicRoomDescriptor;
use crate::game::RoomAfkState;
use crate::runtime::{
    HostResolver, RoseauApplicationEntrypointError, RoseauApplicationEntrypointReport,
    RoseauApplicationLoopRunner, RoseauApplicationRuntime, RoseauBootstrap,
};
use crate::server::ServerSocketBinder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauApplicationEntrypointRunner {
    loop_runner: RoseauApplicationLoopRunner,
}

impl RoseauApplicationEntrypointRunner {
    pub fn new(loop_runner: RoseauApplicationLoopRunner) -> Self {
        Self { loop_runner }
    }

    pub fn loop_runner(&self) -> &RoseauApplicationLoopRunner {
        &self.loop_runner
    }

    pub fn run<B: ServerSocketBinder, C: StorageConnector, E: SqlExecutor, R: HostResolver>(
        &self,
        bootstrap: &RoseauBootstrap,
        binder: &B,
        connector: &C,
        tick_executor: &MySqlApplicationTickExecutor<E>,
        resolver: &R,
        public_rooms: impl IntoIterator<Item = PublicRoomDescriptor>,
        first_connection_id: i32,
        resolved_config_ip: Option<&str>,
        main_server_players: &[(i32, i32)],
        room_afk_states: &mut [RoomAfkState],
    ) -> Result<RoseauApplicationEntrypointReport, RoseauApplicationEntrypointError> {
        let mut prepare_report = RoseauApplicationRuntime::prepare_with_database_connector(
            bootstrap,
            binder,
            connector,
            public_rooms,
            first_connection_id,
            resolved_config_ip,
        )?;

        if !prepare_report.ready() {
            return Ok(RoseauApplicationEntrypointReport::new(prepare_report, None));
        }

        let loop_report = self.loop_runner.run(
            prepare_report
                .application_runtime_mut()
                .expect("ready prepare report has runtime"),
            tick_executor,
            resolver,
            binder,
            main_server_players,
            room_afk_states,
        )?;

        Ok(RoseauApplicationEntrypointReport::new(
            prepare_report,
            Some(loop_report),
        ))
    }
}

#[cfg(test)]
#[path = "roseau_application_entrypoint_runner_tests.rs"]
mod tests;
