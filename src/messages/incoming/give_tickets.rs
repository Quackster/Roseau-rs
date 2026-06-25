use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GiveTickets;

impl IncomingEvent for GiveTickets {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.record(IncomingCommand::SendTickets);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_send_tickets_command() {
        let mut context = IncomingContext::new();
        GiveTickets.handle(&mut context, &NettyRequest::from_content("GIVE_TICKETS"));

        assert_eq!(context.commands(), &[IncomingCommand::SendTickets]);
    }
}
