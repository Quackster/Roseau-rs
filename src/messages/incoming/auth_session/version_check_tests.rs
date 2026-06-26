use super::*;
use crate::protocol::NettyRequest;

#[test]
fn sends_encryption_on_and_secret_key() {
    let mut context = IncomingContext::new();
    VersionCheck.handle(&mut context, &NettyRequest::from_content("VERSIONCHECK"));

    let mut first = context.sent()[0].clone();
    let mut second = context.sent()[1].clone();

    assert_eq!(first.get(), "#ENCRYPTION_ON##");
    assert_eq!(second.get(), "#SECRET_KEY\rABAB##");
}
