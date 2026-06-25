use crate::protocol::{DecodeError, NettyRequest};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NetworkFrameDecoder {
    buffer: Vec<u8>,
}

impl NetworkFrameDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_bytes(
        &mut self,
        bytes: impl AsRef<[u8]>,
    ) -> Result<Vec<NettyRequest>, DecodeError> {
        self.buffer.extend_from_slice(bytes.as_ref());
        self.decode_available()
    }

    pub fn buffered_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    fn decode_available(&mut self) -> Result<Vec<NettyRequest>, DecodeError> {
        let mut requests = Vec::new();

        loop {
            if self.buffer.len() < 4 {
                break;
            }

            let frame_len = match parse_frame_len(&self.buffer[..4]) {
                Ok(frame_len) => frame_len,
                Err(error) => {
                    self.buffer.clear();
                    return Err(error);
                }
            };
            let total_len = 4 + frame_len;

            if self.buffer.len() < total_len {
                break;
            }

            let frame = self.buffer[..total_len].to_vec();
            self.buffer.drain(..total_len);
            requests.push(NettyRequest::decode_frame(&frame)?);
        }

        Ok(requests)
    }
}

fn parse_frame_len(prefix: &[u8]) -> Result<usize, DecodeError> {
    let length_text = std::str::from_utf8(prefix).map_err(|_| DecodeError::InvalidLength)?;
    length_text
        .trim()
        .parse::<usize>()
        .map_err(|_| DecodeError::InvalidLength)
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
