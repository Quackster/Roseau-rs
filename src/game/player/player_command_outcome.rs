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
            9,
        );
        details
    }

    #[test]
    fn maps_retrieve_user_info_to_user_object_packet() {
        let outcome = PlayerCommandOutcome::retrieve_user_info(&details());

        assert_eq!(
            outcome.user_object().unwrap().compose().get(),
            "#USEROBJECT\rname=alice\rfigure=hd-100\remail=alice@example.test\rbirthday=1990-01-01\rphonenumber=+44\rcustomData=hello\rhas_read_agreement=1\rsex=F\rcountry=UK\rhas_special_rights=0\rbadge_type=##"
        );
        assert!(outcome.ph_tickets().is_none());
    }

    #[test]
    fn maps_send_tickets_to_ticket_packet() {
        let outcome = PlayerCommandOutcome::send_tickets(&details());

        assert!(outcome.user_object().is_none());
        assert_eq!(
            outcome.ph_tickets().unwrap().compose().get(),
            "#PH_TICKETS 9##"
        );
    }
}
