use super::messenger_send_message::*;
use crate::protocol::NettyRequest;

#[test]
fn records_send_message_command() {
    let mut context = IncomingContext::new();
    MessengerSendMessage.handle(
        &mut context,
        &NettyRequest::from_content("MESSENGER_SENDMSG 1 2\rhello\rthere"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::SendMessengerMessage {
            receiver_ids: vec![1, 2],
            message: "hello\nthere".to_owned(),
        }]
    );
}
