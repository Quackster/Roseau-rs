use super::*;

#[test]
fn composes_encryption_off_packet() {
    let mut response = EncryptionOff.compose();

    assert_eq!(response.get(), "#ENCRYPTION_OFF##");
}
