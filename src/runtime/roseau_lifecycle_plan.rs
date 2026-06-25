use crate::runtime::{RoseauLifecycleStep, ServerBootstrapPlan};
use crate::util::has_valid_ip_address;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauLifecyclePlan {
    steps: Vec<RoseauLifecycleStep>,
}

impl RoseauLifecyclePlan {
    pub fn from_server_plan(plan: &ServerBootstrapPlan) -> Self {
        let mut steps = vec![
            RoseauLifecycleStep::EnsureConfigFiles,
            RoseauLifecycleStep::StartLogging,
            RoseauLifecycleStep::LoadRuntime,
            RoseauLifecycleStep::ValidateDatabaseEngine {
                engine: plan.database_engine(),
            },
            RoseauLifecycleStep::OpenDatabase {
                engine: plan.database_engine(),
            },
            RoseauLifecycleStep::LoadGame,
            RoseauLifecycleStep::SetupServer,
            RoseauLifecycleStep::ConstructServerHandler {
                class_path: plan.server_class_path().to_owned(),
                bind_ip: plan.bind_ip().to_owned(),
                ports: plan.ports().to_vec(),
            },
        ];

        if !has_valid_ip_address(plan.raw_config_ip()) {
            steps.push(RoseauLifecycleStep::ResolveConfiguredHost {
                host: plan.raw_config_ip().to_owned(),
            });
        }

        steps.push(RoseauLifecycleStep::Listen {
            bind_ip: plan.bind_ip().to_owned(),
            port: plan.server_port(),
        });

        Self { steps }
    }

    pub fn steps(&self) -> &[RoseauLifecycleStep] {
        &self.steps
    }

    pub fn into_steps(self) -> Vec<RoseauLifecycleStep> {
        self.steps
    }
}
