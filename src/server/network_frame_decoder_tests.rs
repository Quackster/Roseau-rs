use super::network_frame_decoder::*;
use crate::protocol::ClientMessage;

#[test]
fn buffers_partial_frame_until_complete() {
    let mut decoder = NetworkFrameDecoder::new();

    assert!(decoder.push_bytes(b"0015LOGIN").unwrap().is_empty());
    assert_eq!(decoder.buffered_len(), 9);

    let requests = decoder.push_bytes(b" user pass").unwrap();

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].get_header(), "LOGIN");
    assert_eq!(requests[0].content(), "user pass");
    assert_eq!(decoder.buffered_len(), 0);
}

#[test]
fn decodes_multiple_frames_from_one_read() {
    let mut decoder = NetworkFrameDecoder::new();

    let requests = decoder.push_bytes(b"0005HELLO0015LOGIN user pass").unwrap();

    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].get_header(), "HELLO");
    assert_eq!(requests[1].get_header(), "LOGIN");
    assert_eq!(requests[1].content(), "user pass");
}

#[test]
fn keeps_trailing_partial_frame_buffered() {
    let mut decoder = NetworkFrameDecoder::new();

    let requests = decoder.push_bytes(b"0005HELLO0015LOGIN").unwrap();

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].get_header(), "HELLO");
    assert_eq!(decoder.buffered_len(), 9);
}

#[test]
fn clears_buffer_when_length_prefix_is_invalid() {
    let mut decoder = NetworkFrameDecoder::new();

    let error = decoder.push_bytes(b"ABCDLOGIN user pass").unwrap_err();

    assert_eq!(error, DecodeError::InvalidLength);
    assert_eq!(decoder.buffered_len(), 0);
}
