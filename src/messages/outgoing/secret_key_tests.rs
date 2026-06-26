use super::*;

#[test]
fn composes_secret_key_packet() {
    let mut response = SecretKey::new("abc").compose();

    assert_eq!(response.get(), "#SECRET_KEY\rabc##");
}
