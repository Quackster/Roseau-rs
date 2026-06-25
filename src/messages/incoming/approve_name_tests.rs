use super::*;
use crate::protocol::NettyRequest;

#[test]
fn sends_name_approved_for_allowed_name() {
    let mut context = IncomingContext::new().username_chars("abcdefghijklmnopqrstuvwxyz0123456789");
    ApproveName.handle(
        &mut context,
        &NettyRequest::from_content("APPROVENAME alice1"),
    );
    let mut response = context.sent()[0].clone();

    assert_eq!(response.get(), "#NAME_APPROVED##");
}

#[test]
fn sends_name_unacceptable_for_reserved_prefix() {
    let mut context = IncomingContext::new();
    ApproveName.handle(
        &mut context,
        &NettyRequest::from_content("APPROVENAME MOD-alice"),
    );
    let mut response = context.sent()[0].clone();

    assert_eq!(response.get(), "#NAME_UNACCEPTABLE##");
}
