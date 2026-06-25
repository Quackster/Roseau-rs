use crate::runtime::RoseauApplicationTickRunReport;

#[derive(Debug, Clone, PartialEq)]
pub struct RoseauApplicationLoopReport {
    tick_reports: Vec<RoseauApplicationTickRunReport>,
    stopped: bool,
}

impl RoseauApplicationLoopReport {
    pub fn new(
        tick_reports: impl Into<Vec<RoseauApplicationTickRunReport>>,
        stopped: bool,
    ) -> Self {
        Self {
            tick_reports: tick_reports.into(),
            stopped,
        }
    }

    pub fn tick_reports(&self) -> &[RoseauApplicationTickRunReport] {
        &self.tick_reports
    }

    pub fn completed_ticks(&self) -> usize {
        self.tick_reports.len()
    }

    pub fn stopped(&self) -> bool {
        self.stopped
    }

    pub fn should_continue(&self) -> bool {
        !self.stopped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{MySqlApplicationTickExecutionReport, SqlExecutionBatchResult};
    use crate::runtime::{
        RoseauApplicationTickExecutionReport, RoseauApplicationTickRunReport,
        RoseauServerLoopOutcome, RoseauStartupRuntimeError,
    };

    #[test]
    fn exposes_tick_reports_and_loop_decision() {
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

        let report = RoseauApplicationLoopReport::new([tick_report], true);

        assert_eq!(report.completed_ticks(), 1);
        assert!(report.stopped());
        assert!(!report.should_continue());
    }
}
