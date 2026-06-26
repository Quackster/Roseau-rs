use super::*;
use crate::protocol::NettyRequest;

#[test]
fn ignores_compatibility_packets() {
    let mut context = IncomingContext::new();

    CompatibilityNoop.handle(
        &mut context,
        &NettyRequest::from_content("STAT /ShockwaveVersion/10.4.1"),
    );

    assert!(context.sent().is_empty());
    assert!(context.commands().is_empty());
}
