use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SplashPosition;

impl IncomingEvent for SplashPosition {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        if !context.is_in_room() {
            return;
        }

        let position = request.get_message_body();
        if position.is_empty() {
            return;
        }

        context.record(IncomingCommand::SplashPosition { position });
    }
}

#[cfg(test)]
#[path = "splash_position_tests.rs"]
mod tests;
