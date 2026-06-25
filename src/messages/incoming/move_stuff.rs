use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MoveStuff;

impl IncomingEvent for MoveStuff {
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

        let rotation = request
            .get_argument(3)
            .and_then(|rotation| rotation.parse::<i32>().ok());

        context.record(IncomingCommand::MoveStuff {
            item_id,
            x,
            y,
            rotation,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_move_stuff_command_with_rotation() {
        let mut context = IncomingContext::new();
        MoveStuff.handle(
            &mut context,
            &NettyRequest::from_content("MOVESTUFF 7 3 4 2"),
        );

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::MoveStuff {
                    item_id: 7,
                    x: 3,
                    y: 4,
                    rotation: Some(2),
                }
            ]
        );
    }

    #[test]
    fn records_move_stuff_command_without_rotation() {
        let mut context = IncomingContext::new();
        MoveStuff.handle(&mut context, &NettyRequest::from_content("MOVESTUFF 7 3 4"));

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::MoveStuff {
                    item_id: 7,
                    x: 3,
                    y: 4,
                    rotation: None,
                }
            ]
        );
    }
}
