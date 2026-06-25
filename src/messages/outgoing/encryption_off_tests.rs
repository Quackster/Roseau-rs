use super::encryption_off::*;

#[test]
fn composes_encryption_off_packet() {
    let mut response = EncryptionOff.compose();

    assert_eq!(response.get(), "#ENCRYPTION_OFF##");
}
