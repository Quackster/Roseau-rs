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
#[path = "chat_tests.rs"]
mod tests;
