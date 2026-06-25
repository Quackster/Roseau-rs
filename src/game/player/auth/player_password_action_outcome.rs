use crate::game::player::{
    PlayerLoginOutcome, PlayerProfileUpdateOutcome, PlayerRegistrationOutcome,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerPasswordActionOutcome {
    Login(PlayerLoginOutcome),
    Registration(PlayerRegistrationOutcome),
    ProfileUpdate(PlayerProfileUpdateOutcome),
}

impl PlayerPasswordActionOutcome {
    pub fn login(&self) -> Option<&PlayerLoginOutcome> {
        match self {
            Self::Login(outcome) => Some(outcome),
            Self::Registration(_) | Self::ProfileUpdate(_) => None,
        }
    }

    pub fn registration(&self) -> Option<PlayerRegistrationOutcome> {
        match self {
            Self::Registration(outcome) => Some(*outcome),
            Self::Login(_) | Self::ProfileUpdate(_) => None,
        }
    }

    pub fn profile_update(&self) -> Option<&PlayerProfileUpdateOutcome> {
        match self {
            Self::ProfileUpdate(outcome) => Some(outcome),
            Self::Login(_) | Self::Registration(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_typed_outcome_accessors() {
        let login = PlayerPasswordActionOutcome::Login(PlayerLoginOutcome::failed());
        let registration =
            PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::Created);
        let profile =
            PlayerPasswordActionOutcome::ProfileUpdate(PlayerProfileUpdateOutcome::Ignored);

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
}
