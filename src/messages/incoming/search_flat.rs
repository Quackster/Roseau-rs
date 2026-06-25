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
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_flat_name_search_query() {
        let mut context = IncomingContext::new();
        SearchFlat.handle(
            &mut context,
            &NettyRequest::from_content("SEARCHFLAT //cafe/"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::SearchFlat {
                query: "cafe".to_owned(),
            }]
        );
    }

    #[test]
    fn ignores_too_short_flat_name_search_query() {
        let mut context = IncomingContext::new();
        SearchFlat.handle(&mut context, &NettyRequest::from_content("SEARCHFLAT //a/"));

        assert!(context.commands().is_empty());
    }
}
