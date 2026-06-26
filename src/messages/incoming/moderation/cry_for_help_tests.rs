use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_raw_distress_message_when_in_room() {
    let mut context = IncomingContext::new().in_room(true);
    CryForHelp.handle(
        &mut context,
        &NettyRequest::from_content("CRYFORHELP /Lobby;help"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::CryForHelp {
            message: "/Lobby;help".to_owned(),
        }]
    );
}

#[test]
fn ignores_distress_message_outside_room() {
    let mut context = IncomingContext::new();
    CryForHelp.handle(
        &mut context,
        &NettyRequest::from_content("CRYFORHELP /Lobby;help"),
    );

    assert!(context.commands().is_empty());
}
