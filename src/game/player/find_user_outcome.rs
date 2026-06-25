use crate::game::player::PlayerDetails;
use crate::messages::outgoing::{MemberInfo, NoSuchUser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindUserOutcome {
    Found {
        name: String,
        greeting: String,
        last_seen: String,
        location: String,
        figure: String,
    },
    Missing,
}

impl FindUserOutcome {
    pub fn found(
        details: &PlayerDetails,
        last_seen: impl Into<String>,
        location: impl Into<String>,
    ) -> Self {
        Self::Found {
            name: details.username().to_owned(),
            greeting: details.personal_greeting().to_owned(),
            last_seen: last_seen.into(),
            location: location.into(),
            figure: details.figure().to_owned(),
        }
    }

    pub fn member_info(&self) -> Option<MemberInfo> {
        match self {
            Self::Found {
                name,
                greeting,
                last_seen,
                location,
                figure,
            } => Some(MemberInfo::new(name, greeting, last_seen, location, figure)),
            Self::Missing => None,
        }
    }

    pub fn no_such_user(&self) -> Option<NoSuchUser> {
        matches!(self, Self::Missing).then_some(NoSuchUser)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::OutgoingMessage;

    fn details() -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_full(
            7,
            "alice",
            "hello",
            "hd-100",
            "",
            "alice@example.test",
            1,
            50,
            "F",
            "UK",
            "",
            "1990-01-01",
            1234,
            "welcome",
            3,
        );
        details
    }

    #[test]
    fn maps_found_player_to_member_info_packet() {
        let outcome = FindUserOutcome::found(&details(), "now", "On Hotel View");

        assert_eq!(
            outcome.member_info().unwrap().compose().get(),
            "#MEMBERINFO \ralice\rwelcome\rnow\rOn Hotel View\rhd-100##"
        );
        assert!(outcome.no_such_user().is_none());
    }

    #[test]
    fn maps_missing_player_to_no_such_user_packet() {
        let outcome = FindUserOutcome::Missing;

        assert!(outcome.member_info().is_none());
        assert_eq!(
            outcome.no_such_user().unwrap().compose().get(),
            "#NOSUCHUSER##"
        );
    }
}
