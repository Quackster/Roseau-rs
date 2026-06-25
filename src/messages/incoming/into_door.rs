use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IntoDoor;

impl IncomingEvent for IntoDoor {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Ok(item_id) = request.get_message_body().parse::<i32>() else {
            return;
        };

        if context.can_enter_door_item(item_id) {
            context.record(IncomingCommand::EnterDoor { item_id });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_enter_door_command() {
        let mut context = IncomingContext::new().enterable_door_item(77);
        IntoDoor.handle(&mut context, &NettyRequest::from_content("IntoDoor 77"));

        assert_eq!(
            context.commands(),
            &[IncomingCommand::EnterDoor { item_id: 77 }]
        );
    }

    #[test]
    fn ignores_unvalidated_door_items() {
        let mut context = IncomingContext::new();
        IntoDoor.handle(&mut context, &NettyRequest::from_content("IntoDoor 77"));

        assert!(context.commands().is_empty());
    }

    #[test]
    fn ignores_non_enterable_room_items() {
        let mut context = IncomingContext::new().enterable_door_item(88);
        IntoDoor.handle(&mut context, &NettyRequest::from_content("IntoDoor 77"));

        assert!(context.commands().is_empty());
    }
}
