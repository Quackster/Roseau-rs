use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MyPersistentMessage {
    message: String,
}

impl MyPersistentMessage {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl OutgoingMessage for MyPersistentMessage {
    fn write(&self, response: &mut NettyResponse) {
        response.init("MYPERSISTENTMSG");
        response.append_new_argument(&self.message);
    }
}

#[cfg(test)]
#[path = "my_persistent_message_tests.rs"]
mod tests;
