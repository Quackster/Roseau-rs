use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LookTo;

impl IncomingEvent for LookTo {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let (Some(x), Some(y)) = (request.get_argument(0), request.get_argument(1)) else {
            return;
        };

        let (Ok(x), Ok(y)) = (x.parse::<i32>(), y.parse::<i32>()) else {
            return;
        };

        context.record(IncomingCommand::ResetAfkTimer);
        context.record(IncomingCommand::LookTo { x, y });
    }
}
