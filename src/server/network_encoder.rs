use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NetworkEncoder;

impl NetworkEncoder {
    pub fn encode_text(message: &str) -> Vec<u8> {
        Self::encode_latin1(message)
    }

    pub fn encode_response(response: &mut NettyResponse) -> Vec<u8> {
        Self::encode_latin1(&response.get())
    }

    pub fn encode_message(message: &impl OutgoingMessage) -> Vec<u8> {
        let mut response = message.compose();
        Self::encode_response(&mut response)
    }

    fn encode_latin1(message: &str) -> Vec<u8> {
        message
            .chars()
            .map(|character| {
                let value = character as u32;
                if value <= u8::MAX as u32 {
                    value as u8
                } else {
                    b'?'
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
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
}
