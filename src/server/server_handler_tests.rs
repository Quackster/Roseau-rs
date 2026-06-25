use super::server_handler::*;
use crate::messages::IncomingCommand;
use crate::protocol::NettyRequest;

#[test]
fn keeps_server_state_messages_and_sessions() {
    let mut handler = ServerHandler::new(vec![30001, 30002], "127.0.0.1");

    handler.set_ip_address("0.0.0.0");
    handler.set_extra_data("hotel");
    handler.open_connection(5);

    assert_eq!(handler.ports(), &[30001, 30002]);
    assert_eq!(handler.ip_address(), "0.0.0.0");
    assert_eq!(handler.extra_data(), Some("hotel"));
    assert!(handler.message_handler().contains_header("VERSIONCHECK"));
    assert!(handler.session_manager().has_session(5));

    handler.close_connection(5);
    handler.clear_extra_data();

    assert!(!handler.session_manager().has_session(5));
    assert_eq!(handler.extra_data(), None);
}

#[test]
fn dispatches_requests_through_owned_message_handler() {
    let handler = ServerHandler::new(vec![30001], "127.0.0.1");
    let context = handler.dispatch_request(
        IncomingContext::new().in_room(true),
        &NettyRequest::from_content("CHAT hello"),
    );

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::Talk {
                mode: "CHAT".to_owned(),
                message: "hello".to_owned(),
            }
        ]
    );
}
