use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_messenger_initialisation_on_main_server_connection() {
    let mut context = IncomingContext::new();
    MessengerInit.handle(&mut context, &NettyRequest::from_content("MESSENGER_INIT"));

    assert_eq!(context.commands(), &[IncomingCommand::InitMessenger]);
}

#[test]
fn records_messenger_initialisation_before_runtime_authentication() {
    let mut context = IncomingContext::new();
    MessengerInit.handle(&mut context, &NettyRequest::from_content("MESSENGER_INIT"));

    assert_eq!(context.commands(), &[IncomingCommand::InitMessenger]);
}

#[test]
fn ignores_messenger_initialisation_off_the_main_server_connection() {
    let mut context = IncomingContext::new().main_server_connection(false);
    MessengerInit.handle(&mut context, &NettyRequest::from_content("MESSENGER_INIT"));

    assert!(context.commands().is_empty());
}
