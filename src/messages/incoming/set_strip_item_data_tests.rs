use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_use_strip_item_command() {
    let mut context = IncomingContext::new();
    SetStripItemData.handle(
        &mut context,
        &NettyRequest::from_content("SETSTRIPITEMDATA ignored\r42"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::UseStripItem { item_id: 42 }]
    );
}
