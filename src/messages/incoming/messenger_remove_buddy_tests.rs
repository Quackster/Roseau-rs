use super::messenger_remove_buddy::*;
use crate::protocol::NettyRequest;

#[test]
fn records_remove_buddy_command() {
    let mut context = IncomingContext::new();
    MessengerRemoveBuddy.handle(
        &mut context,
        &NettyRequest::from_content("MESSENGER_REMOVEBUDDY alice"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::RemoveBuddy {
            username: "alice".to_owned(),
        }]
    );
}
