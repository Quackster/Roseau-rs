use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FlatPropertyByItem;

impl IncomingEvent for FlatPropertyByItem {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let (Some(decoration), Some(item_id)) = (
            request.get_argument_with(1, "/"),
            request.get_argument_with(2, "/"),
        ) else {
            return;
        };

        if !matches!(decoration, "wallpaper" | "floor") {
            return;
        }

        let Ok(item_id) = item_id.parse::<i32>() else {
            return;
        };

        context.record(IncomingCommand::ApplyDecoration {
            decoration: decoration.to_owned(),
            item_id,
        });
    }
}

#[cfg(test)]
#[path = "flat_property_by_item_tests.rs"]
mod tests;
