use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_decline_buddy_command() {
    let mut context = IncomingContext::new();
    MessengerDeclineBuddy.handle(
        &mut context,
        &NettyRequest::from_content("MESSENGER_DECLINEBUDDY alice"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::DeclineBuddy {
            username: "alice".to_owned(),
        }]
    );
}
