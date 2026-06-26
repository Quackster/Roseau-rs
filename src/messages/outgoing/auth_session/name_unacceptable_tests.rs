use super::*;

#[test]
fn composes_name_unacceptable_packet() {
    let mut response = NameUnacceptable.compose();

    assert_eq!(response.get(), "#NAME_UNACCEPTABLE##");
}
