use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RemoveStuff;

impl IncomingEvent for RemoveStuff {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(item_id) = request.get_argument(0) else {
            return;
        };

        let Ok(item_id) = item_id.parse::<i32>() else {
            return;
        };

        context.record(IncomingCommand::RemoveItem { item_id });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_remove_stuff_command() {
        let mut context = IncomingContext::new();
        RemoveStuff.handle(&mut context, &NettyRequest::from_content("REMOVESTUFF 42"));

        assert_eq!(
            context.commands(),
            &[IncomingCommand::RemoveItem { item_id: 42 }]
        );
    }
}
