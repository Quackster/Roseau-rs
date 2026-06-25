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
mod tests {
    use super::*;
    use crate::messages::OutgoingMessage;

    const ALLOWED: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

    #[test]
    fn approves_allowed_names_and_packet() {
        let approval = PlayerNameApproval::evaluate("alice1", ALLOWED);
        let mut response = approval.name_approved().unwrap().compose();

        assert_eq!(approval, PlayerNameApproval::Approved);
        assert!(approval.is_approved());
        assert_eq!(response.get(), "#NAME_APPROVED##");
        assert!(approval.name_unacceptable().is_none());
    }

    #[test]
    fn rejects_reserved_prefix_length_and_characters() {
        for name in [
            "ab",
            "thisnameisfartoolonghere",
            "MOD-alice",
            "M0D-alice",
            "ali!",
        ] {
            let approval = PlayerNameApproval::evaluate(name, ALLOWED);

            assert_eq!(approval, PlayerNameApproval::Unacceptable);
            assert!(!approval.is_approved());
        }
    }

    #[test]
    fn wildcard_allows_any_character_after_java_prefix_and_length_checks() {
        assert!(PlayerNameApproval::evaluate("ali!", "*").is_approved());
        assert!(!PlayerNameApproval::evaluate("MOD-alice", "*").is_approved());
    }

    #[test]
    fn maps_unacceptable_name_to_packet() {
        let approval = PlayerNameApproval::evaluate("bad!", ALLOWED);
        let mut response = approval.name_unacceptable().unwrap().compose();

        assert_eq!(response.get(), "#NAME_UNACCEPTABLE##");
        assert!(approval.name_approved().is_none());
    }
}
