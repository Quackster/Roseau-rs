use super::*;
use crate::messages::OutgoingMessage;

#[test]
fn maps_applied_decoration_to_flat_property_packet() {
    let outcome = RoomDecorationOutcome::applied("wallpaper", "101");
    let mut response = outcome.flat_property_packet().unwrap().compose();

    assert_eq!(response.get(), "#FLATPROPERTY\rwallpaper/101##");
}

#[test]
fn ignored_decoration_has_no_packet() {
    assert!(RoomDecorationOutcome::Ignored
        .flat_property_packet()
        .is_none());
}
