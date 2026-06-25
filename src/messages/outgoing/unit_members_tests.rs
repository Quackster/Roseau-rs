use super::unit_members::*;

#[test]
fn composes_unit_members_packet() {
    let mut response = UnitMembers::new(["alice", "bob"]).compose();

    assert_eq!(response.get(), "#UNITMEMBERS\ralice\rbob##");
}
