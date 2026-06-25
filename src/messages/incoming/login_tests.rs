use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_login_command() {
    let mut context = IncomingContext::new();
    Login.handle(
        &mut context,
        &NettyRequest::from_content("LOGIN alice secret"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::Login {
            username: "alice".to_owned(),
            password: "secret".to_owned(),
            room_login: false,
        }]
    );
}

#[test]
fn records_room_login_command_when_extra_argument_is_present() {
    let mut context = IncomingContext::new();
    Login.handle(
        &mut context,
        &NettyRequest::from_content("LOGIN alice secret room"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::Login {
            username: "alice".to_owned(),
            password: "secret".to_owned(),
            room_login: true,
        }]
    );
}

#[test]
fn sends_error_when_credentials_are_incomplete() {
    let mut context = IncomingContext::new();
    Login.handle(&mut context, &NettyRequest::from_content("LOGIN alice"));

    let mut response = context.sent()[0].clone();
    assert_eq!(response.get(), "#ERROR Login incorrect##");
    assert!(context.commands().is_empty());
}
