use crate::game::player::PlayerDetails;
use crate::messages::outgoing::{PhTickets, UserObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerCommandOutcome {
    UserInfo(PlayerDetails),
    Tickets(i32),
}

impl PlayerCommandOutcome {
    pub fn retrieve_user_info(details: &PlayerDetails) -> Self {
        Self::UserInfo(details.clone())
    }

    pub fn send_tickets(details: &PlayerDetails) -> Self {
        Self::Tickets(details.tickets())
    }

    pub fn user_object(&self) -> Option<UserObject<PlayerDetails>> {
        match self {
            Self::UserInfo(details) => Some(UserObject::new(details.clone())),
            Self::Tickets(_) => None,
        }
    }

    pub fn ph_tickets(&self) -> Option<PhTickets> {
        match self {
            Self::Tickets(tickets) => Some(PhTickets::new(*tickets)),
            Self::UserInfo(_) => None,
        }
    }
}
