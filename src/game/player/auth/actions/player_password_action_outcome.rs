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
#[path = "player_password_action_outcome_tests.rs"]
mod tests;
