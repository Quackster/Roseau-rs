use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PhNoTickets;

impl OutgoingMessage for PhNoTickets {
    fn write(&self, response: &mut NettyResponse) {
        response.init("PH_NOTICKETS");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_ph_no_tickets_packet() {
        let mut response = PhNoTickets.compose();

        assert_eq!(response.get(), "#PH_NOTICKETS##");
    }
}
