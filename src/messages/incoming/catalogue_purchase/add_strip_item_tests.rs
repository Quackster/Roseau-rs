use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_return_item_command() {
    let mut context = IncomingContext::new();
    AddStripItem.handle(
        &mut context,
        &NettyRequest::from_content("ADDSTRIPITEM x y 42"),
    );

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::ReturnItemToInventory { item_id: 42 }
        ]
    );
}
