use super::you_are_owner::*;

#[test]
fn composes_you_are_owner_packet() {
    let mut response = YouAreOwner.compose();

    assert_eq!(response.get(), "#YOUAREOWNER##");
}
