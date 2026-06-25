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
#[path = "network_frame_decoder_tests.rs"]
mod tests;
