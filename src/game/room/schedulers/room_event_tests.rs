use super::room_event::*;

#[test]
fn tracks_java_room_event_tick_counter() {
    let mut event = RoomEvent::new();

    assert!(event.can_tick(2));
    event.increase_ticked();
    assert!(!event.can_tick(2));
    event.increase_ticked();
    assert!(event.can_tick(2));
    assert_eq!(event.ticked(), 2);
}
