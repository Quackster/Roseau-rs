use super::*;

#[test]
fn composes_no_such_user_packet() {
    let mut response = NoSuchUser.compose();

    assert_eq!(response.get(), "#NOSUCHUSER##");
}
