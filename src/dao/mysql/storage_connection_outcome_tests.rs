use super::*;

#[test]
fn maps_connector_results_to_connection_state() {
    let connected = StorageConnectionOutcome::from_result(Ok(()));
    let failed = StorageConnectionOutcome::from_result(Err(DaoError::new("connection refused")));

    assert!(connected.is_connected());
    assert_eq!(connected.error(), None);
    assert!(!failed.is_connected());
    assert_eq!(failed.error(), Some("connection refused"));
}
