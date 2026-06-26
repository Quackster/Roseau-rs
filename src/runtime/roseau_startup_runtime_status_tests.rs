use super::*;
use crate::server::{ServerListenOutcome, ServerListenPlan};

#[test]
fn reports_ready_runtime_status_from_successful_listen() {
    let plan = ServerListenPlan::new("127.0.0.1", vec![37120, 37119]);
    let outcome = ServerListenOutcome::success_for_plan(&plan);
    let status = RoseauStartupRuntimeStatus::from_listen_outcome(&outcome, 2);

    assert!(status.ready());
    assert_eq!(
        status,
        RoseauStartupRuntimeStatus::Ready {
            bind_addresses: vec!["127.0.0.1:37120".to_owned(), "127.0.0.1:37119".to_owned(),],
            active_connections: 2,
        }
    );
    assert_eq!(status.failed_address(), None);
}

#[test]
fn reports_failed_runtime_status_from_failed_listen() {
    let plan = ServerListenPlan::new("127.0.0.1", vec![37120, 37119]);
    let outcome = ServerListenOutcome::failure_for_plan(&plan, "127.0.0.1:37119");
    let status = RoseauStartupRuntimeStatus::from_listen_outcome(&outcome, 0);

    assert!(!status.ready());
    assert_eq!(status.failed_address(), Some("127.0.0.1:37119"));
    assert_eq!(
        status.bind_addresses(),
        &["127.0.0.1:37120".to_owned(), "127.0.0.1:37119".to_owned(),]
    );
}
