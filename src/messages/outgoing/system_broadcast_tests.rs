use super::*;

#[test]
fn composes_system_broadcast_packet() {
    let mut response = SystemBroadcast::new("maintenance").compose();

    assert_eq!(response.get(), "#SYSTEMBROADCAST\rmaintenance##");
}
