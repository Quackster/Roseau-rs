use super::messenger_assign_personal_message::*;
use crate::protocol::NettyRequest;

#[test]
fn filters_and_truncates_personal_message() {
    let mut context = IncomingContext::new();
    MessengerAssignPersonalMessage.handle(
        &mut context,
        &NettyRequest::from_content("MESSENGER_ASSIGNPERSMSG abcdefghijklmnopqrstuvwxyz"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::AssignPersonalMessage {
            message: "abcdefghijklmnopqrstu".to_owned(),
        }]
    );
}
