use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Purchase;

impl IncomingEvent for Purchase {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let call_id = request.get_message_body().replace('/', "");
        if call_id.is_empty() {
            return;
        }

        context.record(IncomingCommand::Purchase { call_id });
    }
}
