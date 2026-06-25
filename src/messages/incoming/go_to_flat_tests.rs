use super::go_to_flat::*;
use crate::protocol::NettyRequest;

#[test]
fn records_go_to_flat_command() {
    let mut context = IncomingContext::new();
    GoToFlat.handle(&mut context, &NettyRequest::from_content("GOTOFLAT"));

    assert_eq!(context.commands(), &[IncomingCommand::GoToFlat]);
}
