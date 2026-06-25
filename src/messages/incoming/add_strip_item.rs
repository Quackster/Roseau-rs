use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AddStripItem;

impl IncomingEvent for AddStripItem {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        context.record(IncomingCommand::ResetAfkTimer);

        let Some(item_id) = request.get_argument(2) else {
            return;
        };

        let Ok(item_id) = item_id.parse::<i32>() else {
            return;
        };

        context.record(IncomingCommand::ReturnItemToInventory { item_id });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_return_item_command() {
        let mut context = IncomingContext::new();
        AddStripItem.handle(
            &mut context,
            &NettyRequest::from_content("ADDSTRIPITEM x y 42"),
        );

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::ReturnItemToInventory { item_id: 42 }
            ]
        );
    }
}
