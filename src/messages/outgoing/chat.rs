use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chat {
    header: String,
    username: String,
    message: String,
}

impl Chat {
    pub fn new(
        header: impl Into<String>,
        username: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            header: header.into(),
            username: username.into(),
            message: message.into(),
        }
    }
}

impl OutgoingMessage for Chat {
    fn write(&self, response: &mut NettyResponse) {
        response.init(&self.header);
        response.append_new_argument(&self.username);
        response.append_argument(&self.message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_chat_packet_with_dynamic_header() {
        let mut response = Chat::new("CHAT", "alice", "hello#there").compose();

        assert_eq!(response.get(), "#CHAT\ralice hello*there##");
    }
}
