use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Move;

impl IncomingEvent for Move {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        if request.get_argument_amount() < 2 {
            return;
        }

        let (Some(x), Some(y)) = (request.get_argument(0), request.get_argument(1)) else {
            return;
        };

        let (Ok(x), Ok(y)) = (x.parse::<i32>(), y.parse::<i32>()) else {
            return;
        };

        context.record(IncomingCommand::WalkTo { x, y });
    }
}
