use crate::protocol::{DecodeError, NettyRequest};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NetworkDecoder;

impl NetworkDecoder {
    pub fn decode(frame: &[u8]) -> Result<NettyRequest, DecodeError> {
        NettyRequest::decode_frame(frame)
    }
}

#[cfg(test)]
#[path = "network_decoder_tests.rs"]
mod tests;
