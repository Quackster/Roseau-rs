use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;
use crate::util::filter_input;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UpdateFlat;

impl IncomingEvent for UpdateFlat {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let (Some(room_id), Some(room_name), Some(room_state), Some(show_owner_name)) = (
            request.get_argument_with(1, "/"),
            request.get_argument_with(2, "/"),
            request.get_argument_with(3, "/"),
            request.get_argument_with(4, "/"),
        ) else {
            return;
        };

        let Ok(room_id) = room_id.parse::<i32>() else {
            return;
        };

        let filtered_room_name = filter_input(room_name);
        let room_name = if filtered_room_name.chars().count() > 2 {
            filtered_room_name
        } else if let Some(current_room_name) = context.current_room_name_value() {
            current_room_name.to_owned()
        } else {
            return;
        };

        let state = match room_state {
            "closed" => 1,
            "password" => 2,
            _ => 0,
        };

        context.record(IncomingCommand::UpdateFlat {
            room_id,
            room_name,
            state,
            show_owner_name: show_owner_name == "1",
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_flat_update() {
        let mut context = IncomingContext::new();
        UpdateFlat.handle(
            &mut context,
            &NettyRequest::from_content("UPDATEFLAT /42/My room/password/1"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::UpdateFlat {
                room_id: 42,
                room_name: "My room".to_owned(),
                state: 2,
                show_owner_name: true,
            }]
        );
    }

    #[test]
    fn falls_back_to_current_room_name_for_short_submitted_name() {
        let mut context = IncomingContext::new().current_room_name("Existing room");
        UpdateFlat.handle(
            &mut context,
            &NettyRequest::from_content("UPDATEFLAT /42/x/closed/0"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::UpdateFlat {
                room_id: 42,
                room_name: "Existing room".to_owned(),
                state: 1,
                show_owner_name: false,
            }]
        );
    }

    #[test]
    fn ignores_short_submitted_name_without_current_room_name() {
        let mut context = IncomingContext::new();
        UpdateFlat.handle(
            &mut context,
            &NettyRequest::from_content("UPDATEFLAT /42/x/closed/0"),
        );

        assert!(context.commands().is_empty());
    }
}
