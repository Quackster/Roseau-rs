use crate::server::{
    ServerListenEffect, ServerListenOutcome, ServerListenPlan, ServerSocketBinder,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerListenEffectExecutor {
    worker_pools_created: bool,
    pipeline_stages: Vec<&'static str>,
    bind_errors: Vec<(String, String)>,
}

impl ServerListenEffectExecutor {
    pub fn new() -> Self {
        Self {
            worker_pools_created: false,
            pipeline_stages: Vec::new(),
            bind_errors: Vec::new(),
        }
    }

    pub fn execute_plan<B: ServerSocketBinder>(
        &mut self,
        plan: &ServerListenPlan,
        binder: &B,
    ) -> ServerListenOutcome {
        let effects = plan.listen_effects();
        self.execute_effects(plan.bind_addresses(), effects, binder)
    }

    pub fn execute_effects<B: ServerSocketBinder>(
        &mut self,
        bind_addresses: Vec<String>,
        effects: impl IntoIterator<Item = ServerListenEffect>,
        binder: &B,
    ) -> ServerListenOutcome {
        let mut failed_address = None;

        for effect in effects {
            match effect {
                ServerListenEffect::CreateCachedWorkerPools => {
                    self.worker_pools_created = true;
                }
                ServerListenEffect::InstallPipelineStage { name } => {
                    self.pipeline_stages.push(name);
                }
                ServerListenEffect::BindAddress { address } => {
                    if failed_address.is_some() {
                        continue;
                    }

                    if let Err(error) = binder.bind(&address) {
                        self.bind_errors.push((address.clone(), error));
                        failed_address = Some(address);
                    }
                }
            }
        }

        ServerListenOutcome::new(bind_addresses, failed_address)
    }

    pub fn worker_pools_created(&self) -> bool {
        self.worker_pools_created
    }

    pub fn pipeline_stages(&self) -> &[&'static str] {
        &self.pipeline_stages
    }

    pub fn bind_errors(&self) -> &[(String, String)] {
        &self.bind_errors
    }
}

impl Default for ServerListenEffectExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "server_listen_effect_executor_tests.rs"]
mod tests;
