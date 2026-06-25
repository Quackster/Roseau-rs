use crate::messages::outgoing::Ok;
use crate::messages::{IncomingContext, IncomingEvent, OutgoingMessage};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StatusOk;

impl IncomingEvent for StatusOk {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.send(Ok.compose());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn sends_ok_packet() {
        let mut context = IncomingContext::new();
        StatusOk.handle(&mut context, &NettyRequest::from_content("STATUSOK"));
        let mut response = context.sent()[0].clone();

        assert_eq!(response.get(), "#OK##");
    }
}
