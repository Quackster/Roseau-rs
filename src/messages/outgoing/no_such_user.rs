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
mod tests {
    use super::*;

    #[test]
    fn composes_no_such_user_packet() {
        let mut response = NoSuchUser.compose();

        assert_eq!(response.get(), "#NOSUCHUSER##");
    }
}
