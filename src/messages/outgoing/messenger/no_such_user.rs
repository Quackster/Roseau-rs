use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoSuchUser;

impl OutgoingMessage for NoSuchUser {
    fn write(&self, response: &mut NettyResponse) {
        response.init("NOSUCHUSER");
    }
}

#[cfg(test)]
#[path = "no_such_user_tests.rs"]
mod tests;
