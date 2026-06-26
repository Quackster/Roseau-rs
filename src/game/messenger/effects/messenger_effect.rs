use crate::messages::outgoing::{BuddyAddRequests, BuddyList};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessengerEffect {
    SendRequests(BuddyAddRequests),
    SendFriends(BuddyList),
    RefreshFriendList {
        user_id: i32,
        offline_user_id: Option<i32>,
    },
}
