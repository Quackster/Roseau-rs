use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JumpPerformance;

impl IncomingEvent for JumpPerformance {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(data) = request.get_argument_with(3, "\r") else {
            return;
        };

        context.record(IncomingCommand::JumpPerformance {
            data: data.to_owned(),
        });
    }
}

#[cfg(test)]
#[path = "jump_performance_tests.rs"]
mod tests;
