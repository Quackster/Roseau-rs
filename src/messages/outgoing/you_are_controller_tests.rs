use super::you_are_controller::*;

#[test]
fn composes_you_are_controller_packet() {
    let mut response = YouAreController.compose();

    assert_eq!(response.get(), "#YOUARECONTROLLER##");
}
