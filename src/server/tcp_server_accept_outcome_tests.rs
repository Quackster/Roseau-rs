use super::*;

#[test]
fn exposes_accepted_connection_and_error_message() {
    let accepted = TcpServerAcceptOutcome::Accepted { connection_id: 9 };
    let error = TcpServerAcceptOutcome::Error {
        message: "listener missing".to_owned(),
    };

    assert_eq!(accepted.accepted_connection_id(), Some(9));
    assert_eq!(accepted.error(), None);
    assert_eq!(error.accepted_connection_id(), None);
    assert_eq!(error.error(), Some("listener missing"));
}
