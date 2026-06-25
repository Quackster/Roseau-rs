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
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_busy_flat_search_multiplier() {
        let mut context = IncomingContext::new();
        SearchBusyFlats.handle(
            &mut context,
            &NettyRequest::from_content("SEARCHBUSYFLATS /2,whatever"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::SearchBusyFlats { multiplier: 2 }]
        );
    }

    #[test]
    fn records_empty_busy_flat_fallback_for_empty_body() {
        let mut context = IncomingContext::new();
        SearchBusyFlats.handle(&mut context, &NettyRequest::from_content("SEARCHBUSYFLATS"));

        assert_eq!(context.commands(), &[IncomingCommand::EmptySearchBusyFlats]);
    }
}
