use super::set_item_data::*;
use crate::protocol::NettyRequest;

#[test]
fn records_set_item_data_command() {
    let mut context = IncomingContext::new();
    SetItemData.handle(
        &mut context,
        &NettyRequest::from_content("SETITEMDATA /42/FFFF31 note"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::SetItemData {
            item_id: 42,
            data: "FFFF31 note".to_owned(),
        }]
    );
}
