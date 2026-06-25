use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlaceStuffFromStrip;

impl IncomingEvent for PlaceStuffFromStrip {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let (Some(item_id), Some(x), Some(y)) = (
            request.get_argument(0),
            request.get_argument(1),
            request.get_argument(2),
        ) else {
            return;
        };

        let (Ok(item_id), Ok(x), Ok(y)) =
            (item_id.parse::<i32>(), x.parse::<i32>(), y.parse::<i32>())
        else {
            return;
        };

        context.record(IncomingCommand::ResetAfkTimer);
        context.record(IncomingCommand::PlaceFloorItemFromInventory {
            item_id,
            x,
            y,
            rotation: 0,
        });
    }
}

#[cfg(test)]
#[path = "place_stuff_from_strip_tests.rs"]
mod tests;
