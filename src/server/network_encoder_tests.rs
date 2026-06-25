use super::*;
use crate::messages::outgoing::Hello;

#[test]
fn encodes_text_as_latin1_bytes() {
    assert_eq!(NetworkEncoder::encode_text("caf\u{e9}"), b"caf\xe9");
}

#[test]
fn encodes_response_after_finalising() {
    let mut response = NettyResponse::with_header("OK");

    assert_eq!(NetworkEncoder::encode_response(&mut response), b"#OK##");
    assert!(response.is_finalised());
}

#[test]
fn encodes_outgoing_message() {
    assert_eq!(NetworkEncoder::encode_message(&Hello), b"#HELLO##");
}
