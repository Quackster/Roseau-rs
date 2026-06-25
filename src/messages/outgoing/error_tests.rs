use super::*;

#[test]
fn composes_error_packet() {
    let mut response = Error::new("missing").compose();

    assert_eq!(response.get(), "#ERROR missing##");
}
