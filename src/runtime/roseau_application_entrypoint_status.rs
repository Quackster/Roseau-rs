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
