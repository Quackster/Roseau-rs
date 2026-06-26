use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlaceItemFromStrip;

impl IncomingEvent for PlaceItemFromStrip {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(item_id) = request.get_argument(0) else {
            return;
        };

        let Ok(item_id) = item_id.parse::<i32>() else {
            return;
        };

        let wall_position = request
            .get_message_body()
            .strip_prefix(&format!("{item_id} "))
            .unwrap_or_default()
            .to_owned();

        if wall_position.is_empty() {
            return;
        }

        context.record(IncomingCommand::ResetAfkTimer);
        context.record(IncomingCommand::PlaceWallItemFromInventory {
            item_id,
            wall_position,
        });
    }
}

#[cfg(test)]
#[path = "place_item_from_strip_tests.rs"]
mod tests;
