use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_remove_stuff_command() {
    let mut context = IncomingContext::new();
    RemoveStuff.handle(&mut context, &NettyRequest::from_content("REMOVESTUFF 42"));

    assert_eq!(
        context.commands(),
        &[IncomingCommand::RemoveItem { item_id: 42 }]
    );
}
