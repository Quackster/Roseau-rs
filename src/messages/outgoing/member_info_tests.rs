use super::*;

#[test]
fn composes_member_info_packet() {
    let mut response =
        MemberInfo::new("alice", "hello", "now", "On Hotel View", "hd-100").compose();

    assert_eq!(
        response.get(),
        "#MEMBERINFO \ralice\rhello\rnow\rOn Hotel View\rhd-100##"
    );
}
