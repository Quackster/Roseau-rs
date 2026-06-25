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
