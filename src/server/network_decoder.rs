use crate::protocol::{DecodeError, NettyRequest};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NetworkDecoder;

impl NetworkDecoder {
    pub fn decode(frame: &[u8]) -> Result<NettyRequest, DecodeError> {
        NettyRequest::decode_frame(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ClientMessage;

    #[test]
    fn delegates_to_protocol_frame_decoder() {
        let request = NetworkDecoder::decode(b"0015LOGIN user pass").unwrap();

        assert_eq!(request.get_header(), "LOGIN");
        assert_eq!(request.content(), "user pass");
    }
}
