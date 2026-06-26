use super::*;

#[test]
fn composes_jump_data_packet() {
    let mut response = JumpData::new("hopper", "x=1").compose();

    assert_eq!(response.get(), "#JUMPDATA\rhopper\rx=1##");
}
