use super::server_listen_effect_executor::*;
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
