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
