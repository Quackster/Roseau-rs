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
