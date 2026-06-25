use super::remove_item::*;
use crate::protocol::NettyRequest;

#[test]
fn records_remove_item_command() {
    let mut context = IncomingContext::new();
    RemoveItem.handle(
        &mut context,
        &NettyRequest::from_content("REMOVEITEM wall/99"),
    );

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::RemoveItem { item_id: 99 }
        ]
    );
}
