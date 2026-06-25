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
        if floor != "first floor" || room_name.chars().count() < 3 {
            return;
        }

        if !matches!(
            room_model,
            "model_a" | "model_b" | "model_c" | "model_d" | "model_e" | "model_f"
        ) {
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
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_create_flat_command() {
        let mut context = IncomingContext::new();
        CreateFlat.handle(
            &mut context,
            &NettyRequest::from_content("CREATEFLAT /first floor/My room/model_a/closed/1"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::CreateFlat {
                floor: "first floor".to_owned(),
                room_name: "My room".to_owned(),
                room_model: "model_a".to_owned(),
                state: 1,
                show_owner_name: true,
            }]
        );
    }
}
