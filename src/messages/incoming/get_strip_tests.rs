use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_inventory_refresh() {
    let mut context = IncomingContext::new();
    GetStrip.handle(&mut context, &NettyRequest::from_content("GETSTRIP floor"));

    assert_eq!(
        context.commands(),
        &[IncomingCommand::RefreshInventory {
            category: "floor".to_owned(),
        }]
    );
}
