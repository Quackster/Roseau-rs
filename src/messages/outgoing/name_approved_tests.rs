use super::*;

#[test]
fn composes_name_approved_packet() {
    let mut response = NameApproved.compose();

    assert_eq!(response.get(), "#NAME_APPROVED##");
}
