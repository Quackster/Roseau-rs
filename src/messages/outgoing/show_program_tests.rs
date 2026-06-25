use super::show_program::*;

#[test]
fn composes_show_program_packet() {
    let mut response = ShowProgram::new(["room", "a", "b"]).compose();

    assert_eq!(response.get(), "#SHOWPROGRAM\rroom a b##");
}
