use super::*;

#[test]
fn composes_ph_tickets_packet() {
    let mut response = PhTickets::new(7).compose();

    assert_eq!(response.get(), "#PH_TICKETS 7##");
}
