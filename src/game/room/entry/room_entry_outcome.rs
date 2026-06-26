use crate::game::room::RoomEffect;
use crate::messages::outgoing::{Error, FlatLetIn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomEntryOutcome {
    LetIn,
    IncorrectPassword,
    Doorbell(Vec<RoomEffect>),
}

impl RoomEntryOutcome {
    pub fn flat_let_in(&self) -> Option<FlatLetIn> {
        matches!(self, Self::LetIn).then_some(FlatLetIn)
    }

    pub fn error(&self) -> Option<Error> {
        matches!(self, Self::IncorrectPassword).then(|| Error::new("Incorrect flat password"))
    }

    pub fn doorbell_effects(&self) -> &[RoomEffect] {
        match self {
            Self::Doorbell(effects) => effects,
            Self::LetIn | Self::IncorrectPassword => &[],
        }
    }
}

#[cfg(test)]
#[path = "room_entry_outcome_tests.rs"]
mod tests;
