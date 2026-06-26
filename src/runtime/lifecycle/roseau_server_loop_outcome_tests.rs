use super::*;
use crate::server::{TcpServerAcceptOutcome, TcpServerTickOutcome};

#[test]
fn continues_for_successful_tick() {
    let tick = TcpServerTickOutcome::new(TcpServerAcceptOutcome::Idle, [], []);
    let outcome = RoseauServerLoopOutcome::from_tick_result(Ok(tick.clone()));

    assert!(outcome.should_continue());
    assert_eq!(outcome.tick(), Some(&tick));
    assert_eq!(outcome.error(), None);
}

#[test]
fn stops_for_startup_runtime_error() {
    let outcome =
        RoseauServerLoopOutcome::from_tick_result(Err(RoseauStartupRuntimeError::NotListening));

    assert!(!outcome.should_continue());
    assert_eq!(outcome.tick(), None);
    assert_eq!(
        outcome.error(),
        Some(&RoseauStartupRuntimeError::NotListening)
    );
}
