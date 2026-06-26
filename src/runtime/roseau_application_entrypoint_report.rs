use crate::dao::mysql::MySqlDaoEffect;
use crate::runtime::{
    RoseauApplicationEntrypointStatus, RoseauApplicationLoopReport, RoseauApplicationPrepareReport,
};
use std::io;

pub struct RoseauApplicationEntrypointReport {
    prepare_report: RoseauApplicationPrepareReport,
    loop_report: Option<RoseauApplicationLoopReport>,
}

impl RoseauApplicationEntrypointReport {
    pub fn new(
        prepare_report: RoseauApplicationPrepareReport,
        loop_report: Option<RoseauApplicationLoopReport>,
    ) -> Self {
        Self {
            prepare_report,
            loop_report,
        }
    }

    pub fn prepare_report(&self) -> &RoseauApplicationPrepareReport {
        &self.prepare_report
    }

    pub fn loop_report(&self) -> Option<&RoseauApplicationLoopReport> {
        self.loop_report.as_ref()
    }

    pub fn ready(&self) -> bool {
        self.status().ready()
    }

    pub fn ran_loop(&self) -> bool {
        self.status().loop_ran()
    }

    pub fn status(&self) -> RoseauApplicationEntrypointStatus {
        RoseauApplicationEntrypointStatus::new(self.prepare_report.readiness(), self.loop_report())
    }

    pub fn log_lines(&self) -> Vec<String> {
        let mut lines = self
            .prepare_report
            .database_report()
            .effects()
            .iter()
            .filter_map(|effect| match effect {
                MySqlDaoEffect::LogLine(line) => Some(line.clone()),
                MySqlDaoEffect::ConnectStorage
                | MySqlDaoEffect::ConstructPlayerDao
                | MySqlDaoEffect::ConstructRoomDao
                | MySqlDaoEffect::ConstructItemDao
                | MySqlDaoEffect::ConstructCatalogueDao
                | MySqlDaoEffect::ConstructInventoryDao
                | MySqlDaoEffect::ConstructNavigatorDao
                | MySqlDaoEffect::ConstructMessengerDao => None,
            })
            .collect::<Vec<_>>();

        if let Some(application_runtime) = self.prepare_report.application_runtime() {
            lines.extend(application_runtime.startup_log_lines().iter().cloned());
        }

        lines
    }

    pub fn write_output_logs(&self) -> io::Result<()> {
        for line in self.log_lines() {
            self.prepare_report.logger().write_output_line(&line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "roseau_application_entrypoint_report_tests.rs"]
mod tests;
