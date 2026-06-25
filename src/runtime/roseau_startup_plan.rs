use crate::runtime::{
    BootstrapError, RoseauLifecyclePlan, RoseauServerFactory, RoseauStartupStatus,
    ServerBootstrapPlan,
};
use crate::server::{ServerListenOutcome, ServerListenPlan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauStartupPlan {
    server_plan: ServerBootstrapPlan,
    lifecycle_plan: RoseauLifecyclePlan,
    listen_plan: ServerListenPlan,
}

impl RoseauStartupPlan {
    pub fn from_server_plan(server_plan: ServerBootstrapPlan) -> Result<Self, BootstrapError> {
        let lifecycle_plan = RoseauLifecyclePlan::from_server_plan(&server_plan);
        let listen_plan = RoseauServerFactory::new().listen_plan(&server_plan)?;

        Ok(Self {
            server_plan,
            lifecycle_plan,
            listen_plan,
        })
    }

    pub fn server_plan(&self) -> &ServerBootstrapPlan {
        &self.server_plan
    }

    pub fn lifecycle_plan(&self) -> &RoseauLifecyclePlan {
        &self.lifecycle_plan
    }

    pub fn listen_plan(&self) -> &ServerListenPlan {
        &self.listen_plan
    }

    pub fn startup_statuses(
        &self,
        outcome: &ServerListenOutcome,
        resolved_config_ip: Option<&str>,
    ) -> Vec<RoseauStartupStatus> {
        vec![
            RoseauStartupStatus::PreparingServer,
            self.server_plan
                .listen_outcome_status(outcome, resolved_config_ip),
        ]
    }

    pub fn startup_log_lines(
        &self,
        outcome: &ServerListenOutcome,
        resolved_config_ip: Option<&str>,
    ) -> Vec<String> {
        self.startup_statuses(outcome, resolved_config_ip)
            .into_iter()
            .map(|status| status.log_line())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::DatabaseEngine;
    use crate::runtime::RoseauLifecycleStep;

    fn server_plan(bind_ip: &str, raw_config_ip: &str) -> ServerBootstrapPlan {
        ServerBootstrapPlan::new(
            bind_ip,
            raw_config_ip,
            37120,
            37119,
            "roseau::server::ServerHandler",
            DatabaseEngine::MySql,
            vec![37120, 37119, 37125],
        )
    }

    #[test]
    fn composes_lifecycle_and_listen_plans_from_server_plan() {
        let startup =
            RoseauStartupPlan::from_server_plan(server_plan("127.0.0.1", "127.0.0.1")).unwrap();

        assert_eq!(startup.server_plan().server_port(), 37120);
        assert_eq!(startup.listen_plan().bind_ip(), "127.0.0.1");
        assert_eq!(
            startup.listen_plan().bind_addresses(),
            vec![
                "127.0.0.1:37120".to_owned(),
                "127.0.0.1:37119".to_owned(),
                "127.0.0.1:37125".to_owned(),
            ]
        );
        assert!(startup
            .lifecycle_plan()
            .steps()
            .contains(&RoseauLifecycleStep::SetupServer));
    }

    #[test]
    fn reports_java_compatible_startup_lines_from_listen_outcome() {
        let startup =
            RoseauStartupPlan::from_server_plan(server_plan("0.0.0.0", "roseau.local")).unwrap();
        let outcome = ServerListenOutcome::success_for_plan(startup.listen_plan());

        assert_eq!(
            startup.startup_log_lines(&outcome, Some("10.0.0.25")),
            vec![
                "Settting up server".to_owned(),
                "Server is listening on 10.0.0.25:37120".to_owned(),
            ]
        );
    }

    #[test]
    fn reports_listen_failure_lines_from_listen_outcome() {
        let startup =
            RoseauStartupPlan::from_server_plan(server_plan("127.0.0.1", "127.0.0.1")).unwrap();
        let outcome =
            ServerListenOutcome::failure_for_plan(startup.listen_plan(), "127.0.0.1:37119");

        assert_eq!(
            startup.startup_log_lines(&outcome, None),
            vec![
                "Settting up server".to_owned(),
                "Server could not listen on 37120:37120, please double check everything is correct in icarus.properties".to_owned(),
            ]
        );
    }
}
