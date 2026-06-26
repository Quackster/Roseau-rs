use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_chat_message_when_in_room() {
    let mut context = IncomingContext::new().in_room(true);
    Talk.handle(
        &mut context,
        &NettyRequest::from_content("CHAT hello\rthere"),
    );

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::Talk {
                mode: "CHAT".to_owned(),
                message: "hello there".to_owned(),
            }
        ]
    );
}

#[test]
fn ignores_chat_message_outside_room() {
    let mut context = IncomingContext::new();
    Talk.handle(&mut context, &NettyRequest::from_content("CHAT hello"));

    assert_eq!(context.commands(), &[IncomingCommand::ResetAfkTimer]);
}
