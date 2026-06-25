use super::look_to::*;
use crate::protocol::NettyRequest;

#[test]
fn records_look_to_command() {
    let mut context = IncomingContext::new();
    LookTo.handle(&mut context, &NettyRequest::from_content("LOOKTO 9 8"));

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::LookTo { x: 9, y: 8 }
        ]
    );
}
