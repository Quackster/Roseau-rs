use super::bad_name::*;

#[test]
fn composes_bad_name_packet() {
    let mut response = BadName.compose();

    assert_eq!(response.get(), "#BADNAME##");
}
