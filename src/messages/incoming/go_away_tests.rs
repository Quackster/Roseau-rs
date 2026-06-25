use super::go_away::*;
use crate::protocol::NettyRequest;

#[test]
fn records_go_away_when_in_room() {
    let mut context = IncomingContext::new().in_room(true);
    GoAway.handle(&mut context, &NettyRequest::from_content("GOAWAY"));

    assert_eq!(context.commands(), &[IncomingCommand::GoAway]);
}
