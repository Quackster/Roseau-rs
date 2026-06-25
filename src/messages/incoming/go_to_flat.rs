use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GoToFlat;

impl IncomingEvent for GoToFlat {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.record(IncomingCommand::GoToFlat);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_go_to_flat_command() {
        let mut context = IncomingContext::new();
        GoToFlat.handle(&mut context, &NettyRequest::from_content("GOTOFLAT"));

        assert_eq!(context.commands(), &[IncomingCommand::GoToFlat]);
    }
}
