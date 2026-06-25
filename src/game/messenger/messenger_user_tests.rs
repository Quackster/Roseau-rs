use super::*;

#[test]
fn offline_users_are_never_in_room() {
    let user = MessengerUser::with_presence(42, false, true);

    assert_eq!(user.user_id(), 42);
    assert!(!user.is_online());
    assert!(!user.in_room());
}
