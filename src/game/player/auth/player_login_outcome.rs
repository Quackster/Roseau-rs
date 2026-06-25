use crate::game::player::{PlayerDetails, PlayerEffect};
use crate::messages::outgoing::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerLoginOutcome {
    Authenticated {
        details: PlayerDetails,
        effects: Vec<PlayerEffect>,
        public_room_lookup_id: Option<i32>,
    },
    Failed,
}

impl PlayerLoginOutcome {
    pub fn authenticated(
        details: &PlayerDetails,
        password: &str,
        room_login: bool,
        server_port: i32,
        base_server_port: i32,
        duplicate_connection_id: Option<i32>,
    ) -> Self {
        let mut details = details.clone();
        details.set_authenticated(true);
        details.set_password(password);

        let mut effects = Vec::new();
        if let Some(connection_id) = duplicate_connection_id {
            effects.push(PlayerEffect::CloseConnection { connection_id });
        }
        effects.push(PlayerEffect::UpdateLastLogin {
            user_id: details.id(),
        });

        Self::Authenticated {
            details,
            effects,
            public_room_lookup_id: room_login.then_some(server_port - base_server_port),
        }
    }

    pub fn failed() -> Self {
        Self::Failed
    }

    pub fn login_error(&self) -> Option<Error> {
        matches!(self, Self::Failed).then(|| Error::new("Login incorrect"))
    }

    pub fn details(&self) -> Option<&PlayerDetails> {
        match self {
            Self::Authenticated { details, .. } => Some(details),
            Self::Failed => None,
        }
    }

    pub fn effects(&self) -> &[PlayerEffect] {
        match self {
            Self::Authenticated { effects, .. } => effects,
            Self::Failed => &[],
        }
    }

    pub fn public_room_lookup_id(&self) -> Option<i32> {
        match self {
            Self::Authenticated {
                public_room_lookup_id,
                ..
            } => *public_room_lookup_id,
            Self::Failed => None,
        }
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
            "mission",
            "figure",
            "pool",
            "alice@example.test",
            1,
            10,
            "F",
            "UK",
            "",
            "1990-01-01",
            1234,
            "hello",
            2,
        );
        details
    }

    #[test]
    fn maps_successful_main_server_login_to_authenticated_details_and_last_login() {
        let outcome =
            PlayerLoginOutcome::authenticated(&details(), "secret", false, 30001, 30001, None);

        let authenticated = outcome.details().unwrap();
        assert!(authenticated.is_authenticated());
        assert_eq!(authenticated.password(), "secret");
        assert_eq!(
            outcome.effects(),
            &[PlayerEffect::UpdateLastLogin { user_id: 7 }]
        );
        assert_eq!(outcome.public_room_lookup_id(), None);
        assert!(outcome.login_error().is_none());
    }

    #[test]
    fn maps_room_login_to_duplicate_close_and_public_room_lookup() {
        let outcome =
            PlayerLoginOutcome::authenticated(&details(), "secret", true, 30045, 30001, Some(11));

        assert_eq!(
            outcome.effects(),
            &[
                PlayerEffect::CloseConnection { connection_id: 11 },
                PlayerEffect::UpdateLastLogin { user_id: 7 },
            ]
        );
        assert_eq!(outcome.public_room_lookup_id(), Some(44));
    }

    #[test]
    fn maps_failed_login_to_java_error_packet() {
        let outcome = PlayerLoginOutcome::failed();
        let mut response = outcome.login_error().unwrap().compose();

        assert_eq!(response.get(), "#ERROR Login incorrect##");
        assert!(outcome.details().is_none());
        assert!(outcome.effects().is_empty());
    }
}
