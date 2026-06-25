use crate::dao::mysql::{MySqlApplicationTickExecutor, SqlExecutor};
use crate::dao::DaoError;
use crate::game::RoomAfkState;
use crate::runtime::{HostResolver, RoseauApplicationLoopReport, RoseauApplicationRuntime};
use crate::server::ServerSocketBinder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauApplicationLoopRunner {
    max_ticks: Option<usize>,
}

impl RoseauApplicationLoopRunner {
    pub fn new() -> Self {
        Self { max_ticks: None }
    }

    pub fn bounded(max_ticks: usize) -> Self {
        Self {
            max_ticks: Some(max_ticks),
        }
    }

    pub fn max_ticks(&self) -> Option<usize> {
        self.max_ticks
    }

    pub fn run<B: ServerSocketBinder, E: SqlExecutor, R: HostResolver>(
        &self,
        application: &mut RoseauApplicationRuntime,
        tick_executor: &MySqlApplicationTickExecutor<E>,
        resolver: &R,
        binder: &B,
        listener_index: usize,
        accept_connection: bool,
        max_bytes: usize,
        main_server_players: &[(i32, i32)],
        room_afk_states: &mut [RoomAfkState],
    ) -> Result<RoseauApplicationLoopReport, DaoError> {
        let mut tick_reports = Vec::new();
        let mut stopped = false;

        loop {
            if self
                .max_ticks
                .is_some_and(|max_ticks| tick_reports.len() >= max_ticks)
            {
                break;
            }

            let report = application.run_tick_and_apply_runtime_plans(
                tick_executor,
                resolver,
                binder,
                listener_index,
                accept_connection,
                max_bytes,
                main_server_players.iter().copied(),
                room_afk_states,
            )?;
            stopped = !report.should_continue();
            tick_reports.push(report);

            if stopped {
                break;
            }
        }

        Ok(RoseauApplicationLoopReport::new(tick_reports, stopped))
    }
}

#[cfg(test)]
#[path = "roseau_application_loop_runner_tests.rs"]
mod tests;
