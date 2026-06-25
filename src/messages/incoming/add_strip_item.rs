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
#[path = "add_strip_item_tests.rs"]
mod tests;
