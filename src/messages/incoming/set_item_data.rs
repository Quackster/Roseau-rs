use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SetItemData;

impl IncomingEvent for SetItemData {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(item_id) = request.get_argument_with(1, "/") else {
            return;
        };

        let Ok(item_id) = item_id.parse::<i32>() else {
            return;
        };

        let data = request
            .get_message_body()
            .replace(&format!("/{item_id}/"), "");

        context.record(IncomingCommand::SetItemData { item_id, data });
    }
}

#[cfg(test)]
#[path = "set_item_data_tests.rs"]
mod tests;
