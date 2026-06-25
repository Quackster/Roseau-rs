use super::order_info::*;

#[test]
fn composes_order_info_packet() {
    let mut response = OrderInfo::new("chair", 3).compose();

    assert_eq!(response.get(), "#ORDERINFO\rchair\r3##");
}
