use super::*;

#[test]
fn finite_status_keeps_duration_and_ticks() {
    let mut status = RoomUserStatus::new("talk", "", false, 5);

    assert_eq!(status.status(), "talk");
    assert_eq!(status.key(), "talk");
    assert_eq!(status.value(), "");
    assert!(!status.is_infinite());
    assert_eq!(status.duration(), 5);

    status.tick();
    assert_eq!(status.duration(), 4);
}

#[test]
fn infinite_status_uses_java_negative_duration() {
    let mut status = RoomUserStatus::new("flatctrl", "1", true, 99);

    assert!(status.is_infinite());
    assert_eq!(status.duration(), -1);

    status.tick();
    assert_eq!(status.duration(), -2);
}
