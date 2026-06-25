use super::messenger_accept_buddy::*;
use crate::protocol::NettyRequest;

#[test]
fn records_accept_buddy_command() {
    let mut context = IncomingContext::new();
    MessengerAcceptBuddy.handle(
        &mut context,
        &NettyRequest::from_content("MESSENGER_ACCEPTBUDDY alice"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::AcceptBuddy {
            username: "alice".to_owned(),
        }]
    );
}
