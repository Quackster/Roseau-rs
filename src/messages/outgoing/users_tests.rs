use super::users::*;

#[test]
fn composes_users_packet() {
    let mut response = Users::new([UserEntry::new(
        "alice",
        "hd-100",
        1,
        2,
        3.5,
        "hello",
        Some("pool"),
    )])
    .compose();

    assert_eq!(
        response.get(),
        "#USERS\r  alice hd-100 1 2 3.5 hello pool##"
    );
}
