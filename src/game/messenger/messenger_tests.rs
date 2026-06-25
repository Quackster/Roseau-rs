use super::messenger::*;
use crate::messages::OutgoingMessage;

fn friend(
    user_id: i32,
    username: &str,
    last_online: i64,
    online: bool,
    initialised: bool,
) -> MessengerFriend {
    MessengerFriend::new(
        user_id,
        username,
        format!("hello {username}"),
        Some(format!("room {username}")),
        last_online,
        online,
        initialised,
    )
}

#[test]
fn tracks_requests_and_friends_by_id() {
    let mut messenger = Messenger::new(1);
    messenger.load(
        vec![friend(2, "bob", 20, false, false)],
        vec![friend(3, "carol", 30, false, false)],
    );

    assert!(messenger.is_friend(2));
    assert!(messenger.has_request(3));
    assert_eq!(messenger.remove_friend(2).unwrap().username(), "bob");
    assert!(!messenger.is_friend(2));
}

#[test]
fn sends_requests_when_available() {
    let mut messenger = Messenger::new(1);
    messenger.load(
        Vec::new(),
        vec![
            friend(2, "bob", 20, false, false),
            friend(3, "carol", 30, false, false),
        ],
    );

    let Some(MessengerEffect::SendRequests(packet)) = messenger.send_requests() else {
        panic!("expected request packet");
    };
    let mut response = packet.compose();

    assert_eq!(response.get(), "#BUDDYADDREQUESTS\r/bob/carol##");
}

#[test]
fn sorts_friends_like_java_before_sending() {
    let mut messenger = Messenger::new(1);
    messenger.load(
        vec![
            friend(2, "late", 50, false, false),
            friend(3, "online", 10, true, true),
            friend(4, "offline", 10, false, false),
        ],
        Vec::new(),
    );

    let MessengerEffect::SendFriends(packet) = messenger.send_friends(Some(3)) else {
        panic!("expected friend packet");
    };
    let mut response = packet.compose();

    assert_eq!(
        response.get(),
        "#BUDDYLIST\r4\toffline\thello offline\r\t10\r3\tonline\thello online\r\t10\r2\tlate\thello late\r\t50##"
    );
}

#[test]
fn status_and_dispose_refresh_initialised_online_friends() {
    let mut messenger = Messenger::new(1);
    messenger.set_initialised(true);
    messenger.load(
        vec![
            friend(2, "bob", 20, true, true),
            friend(3, "carol", 30, true, false),
        ],
        vec![friend(4, "dave", 40, false, false)],
    );

    assert_eq!(
        messenger.status_effects(None),
        vec![MessengerEffect::RefreshFriendList {
            user_id: 2,
            offline_user_id: None,
        }]
    );
    assert_eq!(
        messenger.dispose(),
        vec![MessengerEffect::RefreshFriendList {
            user_id: 2,
            offline_user_id: Some(1),
        }]
    );
    assert!(messenger.friends().is_empty());
    assert!(messenger.requests().is_empty());
    assert!(!messenger.has_initialised());
}
