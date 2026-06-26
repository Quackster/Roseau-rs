use super::*;

#[test]
fn stores_tick_components() {
    let outcome = TcpServerTickOutcome::new(
        TcpServerAcceptOutcome::Accepted { connection_id: 5 },
        [TcpServerStepOutcome::Closed { connection_id: 4 }],
        [4],
    );

    assert_eq!(
        outcome.accept_outcome(),
        &TcpServerAcceptOutcome::Accepted { connection_id: 5 }
    );
    assert_eq!(outcome.accepted_connection_id(), Some(5));
    assert_eq!(outcome.accept_error(), None);
    assert_eq!(
        outcome.read_outcomes(),
        &[TcpServerStepOutcome::Closed { connection_id: 4 }]
    );
    assert_eq!(outcome.removed_connection_ids(), &[4]);
}
