use super::*;
use crate::protocol::NettyRequest;

#[test]
fn sends_encryption_off_and_fixed_secret_key() {
    let mut context = IncomingContext::new();
    VersionCheck.handle(&mut context, &NettyRequest::from_content("VERSIONCHECK"));

    let mut first = context.sent()[0].clone();
    let mut second = context.sent()[1].clone();

    assert_eq!(first.get(), "#ENCRYPTION_OFF##");
    assert_eq!(
        second.get(),
        "#SECRET_KEY\r31vw2swky25q9ko940i8x068ftxrmt0wa3vgj27qtrr3m35rn067o549fl##"
    );
}

#[test]
fn reuses_the_same_secret_key_every_time() {
    let mut context = IncomingContext::new();
    VersionCheck.handle(&mut context, &NettyRequest::from_content("VERSIONCHECK"));
    let mut first = context.sent()[1].clone();

    let mut second_context = IncomingContext::new();
    VersionCheck.handle(&mut second_context, &NettyRequest::from_content("VERSIONCHECK"));
    let mut second = second_context.sent()[1].clone();

    assert_eq!(first.get(), second.get());
}
