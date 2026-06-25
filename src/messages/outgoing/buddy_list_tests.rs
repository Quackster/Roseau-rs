use super::buddy_list::*;

#[test]
fn composes_buddy_list_packet() {
    let mut response = BuddyList::new(
        [
            BuddyListFriend::new(1, "alice", "hi", Some("Cafe"), "now"),
            BuddyListFriend::new(2, "bob", "away", None::<String>, "yesterday"),
        ],
        Some(2),
    )
    .compose();

    assert_eq!(
        response.get(),
        "#BUDDYLIST\r1\talice\thi\rCafe\tnow\r2\tbob\taway\r\tyesterday##"
    );
}
