use super::*;

#[test]
fn composes_encryption_on_packet() {
    let mut response = EncryptionOn.compose();

    assert_eq!(response.get(), "#ENCRYPTION_ON##");
}
