use super::get_order_info::*;
use crate::protocol::NettyRequest;

#[test]
fn records_catalogue_call_id() {
    let mut context = IncomingContext::new();
    GetOrderInfo.handle(
        &mut context,
        &NettyRequest::from_content("GETORDERINFO xxx chair"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::GetOrderInfo {
            call_id: "chair".to_owned(),
        }]
    );
}
