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
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_decoration_item_command() {
        let mut context = IncomingContext::new();
        FlatPropertyByItem.handle(
            &mut context,
            &NettyRequest::from_content("FLATPROPERTYBYITEM /wallpaper/42"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::ApplyDecoration {
                decoration: "wallpaper".to_owned(),
                item_id: 42,
            }]
        );
    }

    #[test]
    fn ignores_unknown_decoration_type() {
        let mut context = IncomingContext::new();
        FlatPropertyByItem.handle(
            &mut context,
            &NettyRequest::from_content("FLATPROPERTYBYITEM /ceiling/42"),
        );

        assert!(context.commands().is_empty());
    }
}
