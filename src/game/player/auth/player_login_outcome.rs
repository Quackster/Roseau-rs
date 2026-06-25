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
