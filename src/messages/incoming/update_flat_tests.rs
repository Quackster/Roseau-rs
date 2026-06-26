use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_flat_update() {
    let mut context = IncomingContext::new();
    UpdateFlat.handle(
        &mut context,
        &NettyRequest::from_content("UPDATEFLAT /42/My room/password/1"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::UpdateFlat {
            room_id: 42,
            room_name: "My room".to_owned(),
            state: 2,
            show_owner_name: true,
        }]
    );
}

#[test]
fn falls_back_to_current_room_name_for_short_submitted_name() {
    let mut context = IncomingContext::new().current_room_name("Existing room");
    UpdateFlat.handle(
        &mut context,
        &NettyRequest::from_content("UPDATEFLAT /42/x/closed/0"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::UpdateFlat {
            room_id: 42,
            room_name: "Existing room".to_owned(),
            state: 1,
            show_owner_name: false,
        }]
    );
}

#[test]
fn ignores_short_submitted_name_without_current_room_name() {
    let mut context = IncomingContext::new();
    UpdateFlat.handle(
        &mut context,
        &NettyRequest::from_content("UPDATEFLAT /42/x/closed/0"),
    );

    assert!(context.commands().is_empty());
}
