use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhTickets {
    tickets: i32,
}

impl PhTickets {
    pub fn new(tickets: i32) -> Self {
        Self { tickets }
    }
}

impl OutgoingMessage for PhTickets {
    fn write(&self, response: &mut NettyResponse) {
        response.init("PH_TICKETS");
        response.append_argument(self.tickets);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_ph_tickets_packet() {
        let mut response = PhTickets::new(7).compose();

        assert_eq!(response.get(), "#PH_TICKETS 7##");
    }
}
