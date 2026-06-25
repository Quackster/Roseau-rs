use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl OutgoingMessage for Error {
    fn write(&self, response: &mut NettyResponse) {
        response.init("ERROR");
        response.append_argument(&self.message);
    }
}
