use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SearchBusyFlats;

impl IncomingEvent for SearchBusyFlats {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let body = request.get_message_body();
        if body.is_empty() {
            context.record(IncomingCommand::EmptySearchBusyFlats);
            return;
        }

        let Some(multiplier) = body
            .replace('/', "")
            .split(',')
            .next()
            .and_then(|part| part.parse::<i32>().ok())
        else {
            return;
        };

        context.record(IncomingCommand::SearchBusyFlats { multiplier });
    }
}

#[cfg(test)]
#[path = "search_busy_flats_tests.rs"]
mod tests;
