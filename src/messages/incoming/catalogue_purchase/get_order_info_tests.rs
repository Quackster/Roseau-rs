use super::*;
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

#[test]
fn records_private_room_catalogue_call_id_shape() {
    let mut context = IncomingContext::new();
    GetOrderInfo.handle(
        &mut context,
        &NettyRequest::from_content("GETORDERINFO /A1 S2S Alex"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::GetOrderInfo {
            call_id: "S2S Alex".to_owned(),
        }]
    );
}

#[test]
fn records_private_room_poster_order_info_call_id_shape() {
    let mut context = IncomingContext::new();
    GetOrderInfo.handle(
        &mut context,
        &NettyRequest::from_content("GETORDERINFO /A1 PEN Alex"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::GetOrderInfo {
            call_id: "PEN Alex".to_owned(),
        }]
    );
}

#[test]
fn records_java_catalogue_page_order_info_shapes() {
    for (content, expected_call_id) in [
        ("GETORDERINFO /A2 STN Alex", "STN Alex"),
        ("GETORDERINFO /A2 PSN Alex", "PSN Alex"),
        ("GETORDERINFO /A1 PYN Alex", "PYN Alex"),
        ("GETORDERINFO /A2 T 101 Alex", "T 101 Alex"),
    ] {
        let mut context = IncomingContext::new();
        GetOrderInfo.handle(&mut context, &NettyRequest::from_content(content));

        assert_eq!(
            context.commands(),
            &[IncomingCommand::GetOrderInfo {
                call_id: expected_call_id.to_owned(),
            }]
        );
    }
}
