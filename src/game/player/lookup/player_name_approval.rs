use crate::messages::outgoing::{NameApproved, NameUnacceptable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerNameApproval {
    Approved,
    Unacceptable,
}

impl PlayerNameApproval {
    pub fn evaluate(name: &str, allowed_chars: &str) -> Self {
        if !is_name_approved(name, allowed_chars) {
            return Self::Unacceptable;
        }

        Self::Approved
    }

    pub fn is_approved(self) -> bool {
        matches!(self, Self::Approved)
    }

    pub fn name_approved(self) -> Option<NameApproved> {
        matches!(self, Self::Approved).then_some(NameApproved)
    }

    pub fn name_unacceptable(self) -> Option<NameUnacceptable> {
        matches!(self, Self::Unacceptable).then_some(NameUnacceptable)
    }
}

fn is_name_approved(name: &str, allowed_chars: &str) -> bool {
    if !(3..=20).contains(&name.chars().count()) {
        return false;
    }

    if name.starts_with("MOD-") || name.starts_with("M0D-") {
        return false;
    }

    if allowed_chars == "*" {
        return true;
    }

    name.chars()
        .all(|character| allowed_chars.contains(character.to_ascii_lowercase()))
}

#[cfg(test)]
#[path = "player_name_approval_tests.rs"]
mod tests;
