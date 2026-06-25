use super::*;

#[test]
fn records_login_password_operation() {
    let action = PasswordAction::verify_login("alice", "secret", true);

    assert_eq!(
        action,
        PasswordAction::VerifyLogin {
            username: "alice".to_owned(),
            password: "secret".to_owned(),
            room_login: true,
        }
    );
    assert_eq!(action.password(), "secret");
}

#[test]
fn records_hashing_operations() {
    assert_eq!(
        PasswordAction::hash_registration(
            "bob",
            "door",
            "bob@example.test",
            "hello",
            "hd-100",
            "M",
            "1990-01-01",
        ),
        PasswordAction::HashRegistration {
            username: "bob".to_owned(),
            password: "door".to_owned(),
            email: "bob@example.test".to_owned(),
            mission: "hello".to_owned(),
            figure: "hd-100".to_owned(),
            sex: "M".to_owned(),
            birthday: "1990-01-01".to_owned(),
        }
    );
    assert_eq!(
        PasswordAction::hash_profile_update(
            Some(7),
            "changed",
            "alice@example.test",
            "hd-200",
            "new mission",
            "F",
        ),
        PasswordAction::HashProfileUpdate {
            user_id: Some(7),
            password: "changed".to_owned(),
            email: "alice@example.test".to_owned(),
            figure: "hd-200".to_owned(),
            mission: "new mission".to_owned(),
            sex: "F".to_owned(),
        }
    );
}

#[test]
fn applies_profile_update_to_existing_details_and_clears_pool_figure_on_sex_change() {
    let mut details = PlayerDetails::new();
    details.fill_full(
        7,
        "alice",
        "old mission",
        "hd-100",
        "pool",
        "old@example.test",
        1,
        50,
        "F",
        "UK",
        "",
        "1990-01-01",
        1234,
        "hello",
        2,
    );
    let action = PasswordAction::hash_profile_update(
        Some(7),
        "changed",
        "alice@example.test",
        "hd-200",
        "new mission",
        "M",
    );

    let updated = action.updated_profile_details(&details).unwrap();

    assert_eq!(updated.password(), "changed");
    assert_eq!(updated.email(), "alice@example.test");
    assert_eq!(updated.figure(), "hd-200");
    assert_eq!(updated.mission(), "new mission");
    assert_eq!(updated.sex(), "M");
    assert_eq!(updated.pool_figure(), "");
}

#[test]
fn refuses_profile_update_for_mismatched_user_id() {
    let mut details = PlayerDetails::new();
    details.fill_basic(8, "alice", "mission", "figure");
    let action = PasswordAction::hash_profile_update(
        Some(7),
        "changed",
        "alice@example.test",
        "hd-200",
        "new mission",
        "F",
    );

    assert_eq!(action.updated_profile_details(&details), None);
}
