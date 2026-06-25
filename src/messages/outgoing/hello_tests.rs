use super::*;

#[test]
fn composes_hello_packet() {
    let mut response = Hello.compose();

    assert_eq!(response.get(), "#HELLO##");
}
