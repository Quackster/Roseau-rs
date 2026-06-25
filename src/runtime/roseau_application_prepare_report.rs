use crate::dao::mysql::MySqlDaoConnectionReport;
use crate::logging::Logger;
use crate::runtime::{RoseauApplicationPrepareReadiness, RoseauApplicationRuntime};

pub struct RoseauApplicationPrepareReport {
    database_report: MySqlDaoConnectionReport,
    application_runtime: Option<RoseauApplicationRuntime>,
    logger: Logger,
}

impl RoseauApplicationPrepareReport {
    pub fn new(
        database_report: MySqlDaoConnectionReport,
        application_runtime: Option<RoseauApplicationRuntime>,
    ) -> Self {
        let logger = application_runtime
            .as_ref()
            .map(|runtime| runtime.runtime().logger().clone())
            .unwrap_or_else(|| Logger::new(false, false, "log"));

        Self::with_logger(database_report, application_runtime, logger)
    }

    pub fn with_logger(
        database_report: MySqlDaoConnectionReport,
        application_runtime: Option<RoseauApplicationRuntime>,
        logger: Logger,
    ) -> Self {
        Self {
            database_report,
            application_runtime,
            logger,
        }
    }

    pub fn database_report(&self) -> &MySqlDaoConnectionReport {
        &self.database_report
    }

    pub fn application_runtime(&self) -> Option<&RoseauApplicationRuntime> {
        self.application_runtime.as_ref()
    }

    pub fn application_runtime_mut(&mut self) -> Option<&mut RoseauApplicationRuntime> {
        self.application_runtime.as_mut()
    }

    pub fn into_application_runtime(self) -> Option<RoseauApplicationRuntime> {
        self.application_runtime
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn readiness(&self) -> RoseauApplicationPrepareReadiness {
        let game_load_readiness = self
            .application_runtime
            .as_ref()
            .map(|runtime| runtime.startup_load_report().readiness());
        let startup_status = self
            .application_runtime
            .as_ref()
            .map(RoseauApplicationRuntime::status);

        RoseauApplicationPrepareReadiness::new(
            self.database_report.connected(),
            game_load_readiness,
            startup_status,
        )
    }

    pub fn ready(&self) -> bool {
        self.readiness().ready()
    }
}
