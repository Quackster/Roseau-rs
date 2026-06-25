use super::sit_command::*;
use crate::game::commands::RoomUserCommandState;

#[test]
fn records_sit_effects_for_stationary_room_user() {
    let context = CommandContext::with_room_user(RoomUserCommandState::new(true, false, 2, 3.7));

    assert_eq!(
        SitCommand.handle(&context, ":sit"),
        vec![
            CommandEffect::RemoveRoomStatus {
                key: "dance".to_owned()
            },
            CommandEffect::SetRoomStatus {
                key: "sit".to_owned(),
                value: " 3".to_owned(),
                infinite: true,
                duration: -1,
            },
            CommandEffect::MarkRoomNeedsUpdate,
        ]
    );
}

#[test]
fn ignores_sit_when_room_user_cannot_sit() {
    for room_user in [
        RoomUserCommandState::new(false, false, 2, 3.0),
        RoomUserCommandState::new(true, true, 2, 3.0),
        RoomUserCommandState::new(true, false, 1, 3.0),
        RoomUserCommandState::new(true, false, 2, 3.0).with_status("sit"),
    ] {
        let context = CommandContext::with_room_user(room_user);
        assert!(SitCommand.handle(&context, ":sit").is_empty());
    }
}
