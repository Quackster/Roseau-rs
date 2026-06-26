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
#[path = "update_flat_tests.rs"]
mod tests;
