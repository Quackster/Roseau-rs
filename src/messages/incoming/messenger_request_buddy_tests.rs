use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_request_buddy_command() {
    let mut context = IncomingContext::new();
    MessengerRequestBuddy.handle(
        &mut context,
        &NettyRequest::from_content("MESSENGER_REQUESTBUDDY alice\rignored"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::RequestBuddy {
            username: "alice".to_owned(),
        }]
    );
}
