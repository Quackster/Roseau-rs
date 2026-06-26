use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_mark_read_command() {
    let mut context = IncomingContext::new();
    MessengerMarkRead.handle(
        &mut context,
        &NettyRequest::from_content("MESSENGER_MARKREAD 77"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::MarkMessengerMessageRead { message_id: 77 }]
    );
}
