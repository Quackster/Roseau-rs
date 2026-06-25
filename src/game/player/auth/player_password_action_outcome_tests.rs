use super::*;

#[test]
fn exposes_typed_outcome_accessors() {
    let login = PlayerPasswordActionOutcome::Login(PlayerLoginOutcome::failed());
    let registration =
        PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::Created);
    let profile = PlayerPasswordActionOutcome::ProfileUpdate(PlayerProfileUpdateOutcome::Ignored);

    assert!(login.login().is_some());
    assert_eq!(login.registration(), None);
    assert!(login.profile_update().is_none());

    assert!(registration.login().is_none());
    assert_eq!(
        registration.registration(),
        Some(PlayerRegistrationOutcome::Created)
    );
    assert!(registration.profile_update().is_none());

    assert!(profile.login().is_none());
    assert_eq!(profile.registration(), None);
    assert!(profile.profile_update().is_some());
}
