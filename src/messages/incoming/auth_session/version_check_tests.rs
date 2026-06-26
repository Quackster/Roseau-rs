use super::*;
use crate::protocol::NettyRequest;

#[test]
fn sends_encryption_on_and_secret_key() {
    let mut context = IncomingContext::new();
    VersionCheck.handle(&mut context, &NettyRequest::from_content("VERSIONCHECK"));

    let mut first = context.sent()[0].clone();
    let mut second = context.sent()[1].clone();

    assert_eq!(first.get(), "#ENCRYPTION_ON##");
    let secret_key = context.rc4_secret_key_value().unwrap();
    assert_eq!(secret_key.len(), 62);
    assert!(secret_key
        .chars()
        .all(|character| character.is_ascii_alphanumeric()));
    assert_eq!(second.get(), format!("#SECRET_KEY\r{secret_key}##"));
}

#[test]
fn generates_fresh_secret_keys() {
    let first = generate_v1_secret_key();
    let second = generate_v1_secret_key();

    assert_ne!(first, second);
}
