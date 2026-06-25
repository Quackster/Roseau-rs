use crate::runtime::{RoseauApplicationLoopReport, RoseauApplicationPrepareReadiness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauApplicationEntrypointStatus {
    prepare_readiness: RoseauApplicationPrepareReadiness,
    loop_ran: bool,
    completed_ticks: usize,
    loop_stopped: Option<bool>,
}

impl RoseauApplicationEntrypointStatus {
    pub fn new(
        prepare_readiness: RoseauApplicationPrepareReadiness,
        loop_report: Option<&RoseauApplicationLoopReport>,
    ) -> Self {
        Self {
            prepare_readiness,
            loop_ran: loop_report.is_some(),
            completed_ticks: loop_report.map_or(0, RoseauApplicationLoopReport::completed_ticks),
            loop_stopped: loop_report.map(RoseauApplicationLoopReport::stopped),
        }
    }

    pub fn ready(&self) -> bool {
        self.prepare_readiness.ready()
    }

    pub fn prepare_readiness(&self) -> &RoseauApplicationPrepareReadiness {
        &self.prepare_readiness
    }

    pub fn loop_ran(&self) -> bool {
        self.loop_ran
    }

    pub fn completed_ticks(&self) -> usize {
        self.completed_ticks
    }

    pub fn loop_stopped(&self) -> Option<bool> {
        self.loop_stopped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{MySqlApplicationTickExecutionReport, SqlExecutionBatchResult};
    use crate::game::{
        GameLoadRuntimeAction, GameLoadRuntimeReport, GameRuntimeSchedulerEffect, GameRuntimeTask,
    };
    use crate::runtime::{
        RoseauApplicationTickExecutionReport, RoseauApplicationTickRunReport,
        RoseauServerLoopOutcome, RoseauStartupRuntimeError, RoseauStartupRuntimeStatus,
    };

    fn ready_prepare() -> RoseauApplicationPrepareReadiness {
        RoseauApplicationPrepareReadiness::new(
            true,
            Some(
                GameLoadRuntimeReport::from_actions(&[
                    GameLoadRuntimeAction::LoadVariables,
                    GameLoadRuntimeAction::LoadRoomManager,
                    GameLoadRuntimeAction::LoadItemManager,
                    GameLoadRuntimeAction::LoadCatalogueManager,
                    GameLoadRuntimeAction::LoadCommandManager,
                    GameLoadRuntimeAction::Scheduler(
                        GameRuntimeSchedulerEffect::ScheduleFixedRate {
                            task: GameRuntimeTask::GameTick,
                            initial_delay_ms: 0,
                            interval_ms: 1_000,
                        },
                    ),
                ])
                .readiness(),
            ),
            Some(RoseauStartupRuntimeStatus::Ready {
                bind_addresses: vec!["127.0.0.1:37120".to_owned()],
                active_connections: 0,
            }),
        )
    }

    #[test]
    fn reports_prepared_entrypoint_before_loop_runs() {
        let status = RoseauApplicationEntrypointStatus::new(ready_prepare(), None);

        assert!(status.ready());
        assert!(status.prepare_readiness().ready());
        assert!(!status.loop_ran());
        assert_eq!(status.completed_ticks(), 0);
        assert_eq!(status.loop_stopped(), None);
    }

    #[test]
    fn reports_loop_execution_and_stop_state() {
        let execution_report = RoseauApplicationTickExecutionReport::from_database_report(
            MySqlApplicationTickExecutionReport::new(SqlExecutionBatchResult::new([]), []),
            "127.0.0.1",
            &crate::game::player::PlayerManager::new(vec![]),
        );
        let tick_report = RoseauApplicationTickRunReport::new(
            execution_report,
            RoseauServerLoopOutcome::Stop {
                error: RoseauStartupRuntimeError::NotListening,
            },
            [],
        );
        let loop_report = RoseauApplicationLoopReport::new([tick_report], true);

        let status = RoseauApplicationEntrypointStatus::new(ready_prepare(), Some(&loop_report));

        assert!(status.ready());
        assert!(status.loop_ran());
        assert_eq!(status.completed_ticks(), 1);
        assert_eq!(status.loop_stopped(), Some(true));
    }
}
