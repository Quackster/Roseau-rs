use super::*;

#[test]
fn composes_ph_no_tickets_packet() {
    let mut response = PhNoTickets.compose();

    assert_eq!(response.get(), "#PH_NOTICKETS##");
}
