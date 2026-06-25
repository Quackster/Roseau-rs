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
