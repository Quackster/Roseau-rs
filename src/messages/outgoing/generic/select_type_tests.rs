use super::*;

#[test]
fn composes_select_type_packet() {
    let mut response = SelectType.compose();

    assert_eq!(response.get(), "#SELECTTYPE\rx##");
}
