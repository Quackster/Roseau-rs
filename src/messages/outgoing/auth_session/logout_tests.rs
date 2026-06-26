use super::*;

#[test]
fn composes_logout_packet() {
    let mut response = Logout::new("alice").compose();

    assert_eq!(response.get(), "#LOGOUT\ralice##");
}
