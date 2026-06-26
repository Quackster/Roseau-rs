use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_walk_command() {
    let mut context = IncomingContext::new();
    Move.handle(&mut context, &NettyRequest::from_content("Move 4 5"));

    assert_eq!(
        context.commands(),
        &[IncomingCommand::WalkTo { x: 4, y: 5 }]
    );
}
