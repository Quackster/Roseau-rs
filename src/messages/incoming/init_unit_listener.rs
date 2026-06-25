use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InitUnitListener;

impl IncomingEvent for InitUnitListener {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.record(IncomingCommand::InitUnitListener);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_init_unit_listener_command() {
        let mut context = IncomingContext::new();
        InitUnitListener.handle(
            &mut context,
            &NettyRequest::from_content("INITUNITLISTENER"),
        );

        assert_eq!(context.commands(), &[IncomingCommand::InitUnitListener]);
    }
}
