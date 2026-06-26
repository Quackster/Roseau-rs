use crate::protocol::ClientMessage;

use super::IncomingContext;

pub trait IncomingEvent {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage);
}
