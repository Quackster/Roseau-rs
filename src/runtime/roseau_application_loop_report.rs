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
