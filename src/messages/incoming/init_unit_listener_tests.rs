use super::init_unit_listener::*;
use crate::protocol::NettyRequest;

#[test]
fn records_init_unit_listener_command() {
    let mut context = IncomingContext::new();
    InitUnitListener.handle(
        &mut context,
        &NettyRequest::from_content("INITUNITLISTENER"),
    );

    assert_eq!(context.commands(), &[IncomingCommand::InitUnitListener]);
}
