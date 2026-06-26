use super::*;

#[test]
fn composes_ok_packet() {
    let mut response = Ok.compose();

    assert_eq!(response.get(), "#OK##");
}
