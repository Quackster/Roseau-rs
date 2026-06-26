use super::*;
use crate::protocol::NettyRequest;

#[test]
fn sends_ok_packet() {
    let mut context = IncomingContext::new();
    StatusOk.handle(&mut context, &NettyRequest::from_content("STATUSOK"));
    let mut response = context.sent()[0].clone();

    assert_eq!(response.get(), "#OK##");
}
