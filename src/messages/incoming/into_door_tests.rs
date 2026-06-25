use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_enter_door_command() {
    let mut context = IncomingContext::new().enterable_door_item(77);
    IntoDoor.handle(&mut context, &NettyRequest::from_content("IntoDoor 77"));

    assert_eq!(
        context.commands(),
        &[IncomingCommand::EnterDoor { item_id: 77 }]
    );
}

#[test]
fn ignores_unvalidated_door_items() {
    let mut context = IncomingContext::new();
    IntoDoor.handle(&mut context, &NettyRequest::from_content("IntoDoor 77"));

    assert!(context.commands().is_empty());
}

#[test]
fn ignores_non_enterable_room_items() {
    let mut context = IncomingContext::new().enterable_door_item(88);
    IntoDoor.handle(&mut context, &NettyRequest::from_content("IntoDoor 77"));

    assert!(context.commands().is_empty());
}
