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
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_floor_item_placement() {
        let mut context = IncomingContext::new();
        PlaceStuffFromStrip.handle(
            &mut context,
            &NettyRequest::from_content("PLACESTUFFFROMSTRIP 42 5 7"),
        );

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::PlaceFloorItemFromInventory {
                    item_id: 42,
                    x: 5,
                    y: 7,
                    rotation: 0,
                }
            ]
        );
    }
}
