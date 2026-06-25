use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AddItem;

impl IncomingEvent for AddItem {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        context.record(IncomingCommand::ResetAfkTimer);

        let (Some(sprite), Some(wall_position), Some(extra_data)) = (
            request.get_argument_with(1, "/"),
            request.get_argument_with(2, "/"),
            request.get_argument_with(3, "/"),
        ) else {
            return;
        };

        context.record(IncomingCommand::AddWallItem {
            sprite: sprite.to_owned(),
            wall_position: wall_position.to_owned(),
            extra_data: extra_data.to_owned(),
        });
    }
}
