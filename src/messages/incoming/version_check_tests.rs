use super::version_check::*;
use crate::protocol::NettyRequest;

#[test]
fn sends_encryption_off_and_secret_key() {
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
