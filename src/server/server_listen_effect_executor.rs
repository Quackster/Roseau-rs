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
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, Default)]
    struct RecordingBinder {
        failing_address: Option<String>,
        bound_addresses: RefCell<Vec<String>>,
    }

    impl RecordingBinder {
        fn with_failure(address: &str) -> Self {
            Self {
                failing_address: Some(address.to_owned()),
                bound_addresses: RefCell::new(Vec::new()),
            }
        }

        fn bound_addresses(&self) -> Vec<String> {
            self.bound_addresses.borrow().clone()
        }
    }

    impl ServerSocketBinder for RecordingBinder {
        fn bind(&self, address: &str) -> Result<(), String> {
            if self.failing_address.as_deref() == Some(address) {
                return Err("address already in use".to_owned());
            }

            self.bound_addresses.borrow_mut().push(address.to_owned());
            Ok(())
        }
    }

    #[test]
    fn executes_listen_effects_in_java_netty_order() {
        let plan = ServerListenPlan::new("127.0.0.1", vec![37120, 37119]);
        let binder = RecordingBinder::default();
        let mut executor = ServerListenEffectExecutor::new();

        let outcome = executor.execute_plan(&plan, &binder);

        assert!(outcome.listened());
        assert!(executor.worker_pools_created());
        assert_eq!(
            executor.pipeline_stages(),
            &["encoder", "decoder", "handler"]
        );
        assert_eq!(
            binder.bound_addresses(),
            vec!["127.0.0.1:37120".to_owned(), "127.0.0.1:37119".to_owned()]
        );
        assert!(executor.bind_errors().is_empty());
    }

    #[test]
    fn records_first_bind_failure_and_stops_later_binds() {
        let plan = ServerListenPlan::new("127.0.0.1", vec![37120, 37119, 37125]);
        let binder = RecordingBinder::with_failure("127.0.0.1:37119");
        let mut executor = ServerListenEffectExecutor::new();

        let outcome = executor.execute_plan(&plan, &binder);

        assert!(!outcome.listened());
        assert_eq!(outcome.failed_address(), Some("127.0.0.1:37119"));
        assert_eq!(binder.bound_addresses(), vec!["127.0.0.1:37120"]);
        assert_eq!(
            executor.bind_errors(),
            &[(
                "127.0.0.1:37119".to_owned(),
                "address already in use".to_owned()
            )]
        );
        assert_eq!(outcome.bind_addresses().len(), 3);
    }
}
