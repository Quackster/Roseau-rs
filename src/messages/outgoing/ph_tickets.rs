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
#[path = "ph_tickets_tests.rs"]
mod tests;
