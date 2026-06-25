use super::name_unacceptable::*;

#[test]
fn composes_name_unacceptable_packet() {
    let mut response = NameUnacceptable.compose();

    assert_eq!(response.get(), "#NAME_UNACCEPTABLE##");
}
