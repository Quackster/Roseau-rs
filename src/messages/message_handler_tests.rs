use super::*;
use crate::messages::IncomingCommand;
use crate::protocol::NettyRequest;

#[test]
fn registers_java_message_headers() {
    let handler = MessageHandler::new();

    assert_eq!(handler.len(), 64);
    assert!(handler.contains_header("VERSIONCHECK"));
    assert!(handler.contains_header("MESSENGERINIT"));
    assert!(handler.contains_header("SETFLATINFO"));
    assert!(handler.contains_header("SPLASH_POSITION"));
    assert!(handler.contains_header("CHAT"));
    assert!(handler.contains_header("WHISPER"));
}

#[test]
fn dispatches_request_to_registered_handler() {
    let handler = MessageHandler::new();
    let context = handler.handle_request(
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

#[test]
fn ignores_unknown_header() {
    let handler = MessageHandler::new();
    let context = handler.handle_request(
        IncomingContext::new().in_room(true),
        &NettyRequest::from_content("UNKNOWN hello"),
    );

    assert!(context.commands().is_empty());
    assert!(context.sent().is_empty());
}
