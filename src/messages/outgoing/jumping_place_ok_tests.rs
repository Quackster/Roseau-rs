use super::jumping_place_ok::*;

#[test]
fn composes_jumping_place_ok_packet() {
    let mut response = JumpingPlaceOk.compose();

    assert_eq!(response.get(), "#JUMPINGPLACE_OK##");
}
