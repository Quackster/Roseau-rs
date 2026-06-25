use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_purchase_call_id_without_slashes() {
    let mut context = IncomingContext::new();
    Purchase.handle(
        &mut context,
        &NettyRequest::from_content("PURCHASE /chair_blue"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::Purchase {
            call_id: "chair_blue".to_owned(),
        }]
    );
}
