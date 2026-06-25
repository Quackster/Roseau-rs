use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RemoveItem;

impl IncomingEvent for RemoveItem {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(item_id) = request.get_argument_with(1, "/") else {
            return;
        };

        let Ok(item_id) = item_id.parse::<i32>() else {
            return;
        };

        context.record(IncomingCommand::ResetAfkTimer);
        context.record(IncomingCommand::RemoveItem { item_id });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_remove_item_command() {
        let mut context = IncomingContext::new();
        RemoveItem.handle(
            &mut context,
            &NettyRequest::from_content("REMOVEITEM wall/99"),
        );

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::RemoveItem { item_id: 99 }
            ]
        );
    }
}
