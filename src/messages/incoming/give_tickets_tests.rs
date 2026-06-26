use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_send_tickets_command() {
    let mut context = IncomingContext::new();
    GiveTickets.handle(&mut context, &NettyRequest::from_content("GIVE_TICKETS"));

    assert_eq!(context.commands(), &[IncomingCommand::SendTickets]);
}
