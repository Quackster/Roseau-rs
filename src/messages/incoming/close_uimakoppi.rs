use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CloseUimakoppi;

impl IncomingEvent for CloseUimakoppi {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        if !context.is_in_room() {
            return;
        }

        context.record(IncomingCommand::ClosePoolChangeBooth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_close_pool_change_booth_command_when_in_room() {
        let mut context = IncomingContext::new().in_room(true);
        CloseUimakoppi.handle(&mut context, &NettyRequest::from_content("CLOSE_UIMAKOPPI"));

        assert_eq!(context.commands(), &[IncomingCommand::ClosePoolChangeBooth]);
    }

    #[test]
    fn ignores_close_pool_change_booth_outside_room() {
        let mut context = IncomingContext::new();
        CloseUimakoppi.handle(&mut context, &NettyRequest::from_content("CLOSE_UIMAKOPPI"));

        assert!(context.commands().is_empty());
    }
}
