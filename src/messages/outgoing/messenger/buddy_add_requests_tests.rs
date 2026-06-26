use super::*;

#[test]
fn composes_buddy_add_requests_packet() {
    let mut response = BuddyAddRequests::new(["alice", "bob"]).compose();

    assert_eq!(response.get(), "#BUDDYADDREQUESTS\r/alice/bob##");
}
