use crate::dao::mysql::{MySqlApplicationTickExecutor, SqlExecutor};
use crate::dao::DaoError;
use crate::game::RoomAfkState;
use crate::runtime::{
    HostResolver, RoseauApplicationRuntime, RoseauApplicationTickExecutionReport,
    RoseauApplicationTickRunReport,
};
use crate::server::ServerSocketBinder;

impl RoseauApplicationRuntime {
    pub fn run_tick_execution_report<B: ServerSocketBinder, E: SqlExecutor>(
        &mut self,
        tick_executor: &MySqlApplicationTickExecutor<E>,
        binder: &B,
        main_server_players: impl IntoIterator<Item = (i32, i32)>,
        room_afk_states: &mut [RoomAfkState],
    ) -> Result<RoseauApplicationTickExecutionReport, DaoError> {
        let outcome = self.run_tick(binder, main_server_players, room_afk_states);
        let database_report = tick_executor.execute_tick_report(&outcome)?;
        let raw_config_ip = self
            .startup_runtime()
            .startup_plan()
            .server_plan()
            .raw_config_ip();

        Ok(RoseauApplicationTickExecutionReport::from_database_report(
            database_report,
            raw_config_ip,
            self.game().player_manager(),
        ))
    }

    pub fn run_tick_and_apply_runtime_plans<
        B: ServerSocketBinder,
        E: SqlExecutor,
        R: HostResolver,
    >(
        &mut self,
        tick_executor: &MySqlApplicationTickExecutor<E>,
        resolver: &R,
        binder: &B,
        main_server_players: impl IntoIterator<Item = (i32, i32)>,
        room_afk_states: &mut [RoomAfkState],
    ) -> Result<RoseauApplicationTickRunReport, DaoError> {
        let outcome = self.run_tick(binder, main_server_players, room_afk_states);
        let server_outcome = outcome.server_outcome().clone();
        let database_report = tick_executor.execute_tick_report(&outcome)?;
        let raw_config_ip = self
            .startup_runtime()
            .startup_plan()
            .server_plan()
            .raw_config_ip();
        let execution_report = RoseauApplicationTickExecutionReport::from_database_report(
            database_report,
            raw_config_ip,
            self.game().player_manager(),
        );
        let unapplied_runtime_plans =
            self.apply_tick_runtime_plans_with_resolver(&execution_report, resolver);

        Ok(RoseauApplicationTickRunReport::new(
            execution_report,
            server_outcome,
            unapplied_runtime_plans,
        ))
    }
}
