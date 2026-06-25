use super::server_listen_outcome::*;

#[test]
fn records_successful_bind_attempts_for_all_ports() {
    let plan = ServerListenPlan::new("127.0.0.1", vec![37120, 37119, 37125]);

    let outcome = ServerListenOutcome::success_for_plan(&plan);

    assert!(outcome.listened());
    assert_eq!(outcome.failed_address(), None);
    assert_eq!(
        outcome.bind_addresses(),
        &[
            "127.0.0.1:37120".to_owned(),
            "127.0.0.1:37119".to_owned(),
            "127.0.0.1:37125".to_owned(),
        ]
    );
}

#[test]
fn records_failed_bind_address_without_losing_attempt_plan() {
    let plan = ServerListenPlan::new("0.0.0.0", vec![37120, 37119]);

    let outcome = ServerListenOutcome::failure_for_plan(&plan, "0.0.0.0:37119");

    assert!(!outcome.listened());
    assert_eq!(outcome.failed_address(), Some("0.0.0.0:37119"));
    assert_eq!(outcome.bind_addresses().len(), 2);
}
