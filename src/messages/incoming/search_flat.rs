use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SearchFlat;

impl IncomingEvent for SearchFlat {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let body = request.get_message_body();
        let query = body
            .chars()
            .skip(2)
            .take(body.chars().count().saturating_sub(3))
            .collect::<String>();

        if query.chars().count() > 1 {
            context.record(IncomingCommand::SearchFlat { query });
        }
    }
}

#[cfg(test)]
#[path = "search_flat_tests.rs"]
mod tests;
