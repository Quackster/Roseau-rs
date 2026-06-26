use crate::protocol::{ClientMessage, NettyRequest, NettyResponse, SerializableObject};

struct ExampleObject;

impl SerializableObject for ExampleObject {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_argument("object");
    }
}

#[test]
fn request_splits_header_from_body() {
    let request = NettyRequest::from_content("LOGIN user pass");

    assert_eq!(request.get_header(), "LOGIN");
    assert_eq!(request.content(), "user pass");
    assert_eq!(request.get_argument_amount(), 2);
    assert_eq!(request.get_argument(0), Some("user"));
    assert_eq!(request.get_argument(1), Some("pass"));
}

#[test]
fn request_allows_empty_body() {
    let request = NettyRequest::from_content("STATUSOK");

    assert_eq!(request.get_header(), "STATUSOK");
    assert_eq!(request.content(), "");
    assert_eq!(request.get_argument_amount(), 1);
    assert_eq!(request.get_argument(0), Some(""));
}

#[test]
fn decode_frame_reads_decimal_length_and_latin1_body() {
    let request = NettyRequest::decode_frame(b"0009TALK caf\xe9").unwrap();

    assert_eq!(request.get_header(), "TALK");
    assert_eq!(request.content(), "café");
}

#[test]
fn response_builds_java_compatible_packet() {
    let mut response = NettyResponse::with_header("CHAT");
    response.append_argument("hello#world");
    response.append_new_argument("line");
    response.append_kv_argument("key", "value");
    response.append_object(&ExampleObject);

    assert_eq!(
        response.get(),
        "#CHAT hello*world\rline\rkey=value object##"
    );
    assert!(response.is_finalised());
    assert_eq!(
        response.get(),
        "#CHAT hello*world\rline\rkey=value object##"
    );
}

#[test]
fn body_string_makes_controls_visible() {
    let mut response = NettyResponse::with_header("CHAT");
    response.append_new_argument("line");

    assert_eq!(response.get_body_string(), "#CHAT[13]line##");
}
