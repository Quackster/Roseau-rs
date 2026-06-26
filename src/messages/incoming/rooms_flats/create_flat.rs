use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;
use crate::util::filter_input;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CreateFlat;

impl IncomingEvent for CreateFlat {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let (Some(floor), Some(room_name), Some(room_model), Some(room_state), Some(show_owner)) = (
            request.get_argument_with(1, "/"),
            request.get_argument_with(2, "/"),
            request.get_argument_with(3, "/"),
            request.get_argument_with(4, "/"),
            request.get_argument_with(5, "/"),
        ) else {
            return;
        };

        let room_name = filter_input(room_name);
        if floor != "first floor" {
            context.record(IncomingCommand::CloseUserConnections);
            return;
        }

        if room_name.chars().count() < 3 {
            context.record(IncomingCommand::ClosePublicRoomConnections);
            context.record(IncomingCommand::SendAlert {
                message: "The room name needs to be at least 3 characters long".to_owned(),
            });
            return;
        }

        if !matches!(
            room_model,
            "model_a" | "model_b" | "model_c" | "model_d" | "model_e" | "model_f"
        ) {
            context.record(IncomingCommand::CloseUserConnections);
            return;
        }

        let state = match room_state {
            "closed" => 1,
            "password" => 2,
            _ => 0,
        };

        context.record(IncomingCommand::CreateFlat {
            floor: floor.to_owned(),
            room_name,
            room_model: room_model.to_owned(),
            state,
            show_owner_name: show_owner == "1",
        });
    }
}

#[cfg(test)]
#[path = "create_flat_tests.rs"]
mod tests;
