use super::*;
use crate::messages::outgoing::{BuddyAddRequests, BuddyList, BuddyListFriend};

#[test]
fn plans_direct_messenger_packets_for_connection() {
    let effects = MessengerEffectNetworkPlan::plan_all(
        &[
            MessengerEffect::SendRequests(BuddyAddRequests::new(["alice"])),
            MessengerEffect::SendFriends(BuddyList::new(
                [BuddyListFriend::new(
                    7,
                    "bob",
                    "mission",
                    Some("Room"),
                    "50",
                )],
                None,
            )),
        ],
        42,
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#BUDDYADDREQUESTS\r/alice##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#BUDDYLIST\r7\tbob\tmission\rRoom\t50##".to_owned(),
            },
        ]
    );
}

#[test]
fn leaves_friend_refresh_for_runtime_messenger_lookup() {
    assert!(MessengerEffectNetworkPlan::plan(
        &MessengerEffect::RefreshFriendList {
            user_id: 7,
            offline_user_id: Some(5),
        },
        42,
    )
    .is_empty());
}
