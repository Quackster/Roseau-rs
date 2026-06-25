use super::flat_created::*;

#[test]
fn composes_flat_created_packet() {
    let mut response = FlatCreated::new(12, "Lobby").compose();

    assert_eq!(response.get(), "#FLATCREATED\r12 Lobby##");
}
