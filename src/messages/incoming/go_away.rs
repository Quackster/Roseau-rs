use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GoAway;

impl IncomingEvent for GoAway {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        if context.is_in_room() {
            context.record(IncomingCommand::GoAway);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_go_away_when_in_room() {
        let mut context = IncomingContext::new().in_room(true);
        GoAway.handle(&mut context, &NettyRequest::from_content("GOAWAY"));

        assert_eq!(context.commands(), &[IncomingCommand::GoAway]);
    }
}
