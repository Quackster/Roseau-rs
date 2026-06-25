use super::you_are_not_controller::*;

#[test]
fn composes_you_are_not_controller_packet() {
    let mut response = YouAreNotController.compose();

    assert_eq!(response.get(), "#YOUARENOTCONTROLLER##");
}
