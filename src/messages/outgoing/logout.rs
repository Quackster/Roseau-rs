use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Logout {
    username: String,
}

impl Logout {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
        }
    }
}

impl OutgoingMessage for Logout {
    fn write(&self, response: &mut NettyResponse) {
        response.init("LOGOUT");
        response.append_new_argument(&self.username);
    }
}
