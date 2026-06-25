use crate::dao::in_memory::{InMemoryMessengerDao, InMemoryPlayerDao};
use crate::dao::{CreatePlayer, MessengerDao, PlayerDao};
use crate::game::messenger::{
    Messenger, MessengerCommandExecutor, MessengerCommandOutcome, MessengerFriend,
};

fn create_player(username: &str) -> CreatePlayer {
    CreatePlayer::new(
        username,
        "secret",
        format!("{username}@example.test"),
        "hello",
        "hd=100",
        50,
        "Male",
        "08.08.1997",
    )
}

fn player_dao() -> InMemoryPlayerDao {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&create_player("alice")).unwrap();
    dao.create_player(&create_player("bob")).unwrap();
    dao.create_player(&create_player("carol")).unwrap();
    dao
}

fn friend(user_id: i32, username: &str) -> MessengerFriend {
    MessengerFriend::new(user_id, username, "hello", None::<String>, 0, false, false)
}

#[test]
fn creates_buddy_request_when_target_exists_and_is_not_already_requested() {
    let players = player_dao();
    let messenger_dao = InMemoryMessengerDao::new();
    let messenger = Messenger::new(1);

    let outcome =
        MessengerCommandExecutor::request_buddy(&players, &messenger_dao, &messenger, "bob")
            .unwrap();

    assert_eq!(
        outcome,
        MessengerCommandOutcome::BuddyRequestCreated { to_id: 2 }
    );
    assert!(messenger_dao.request_exists(1, 2).unwrap());
}

#[test]
fn ignores_unknown_self_and_existing_buddy_requests() {
    let players = player_dao();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut messenger = Messenger::new(1);
    messenger.load(Vec::new(), vec![friend(2, "bob")]);

    assert_eq!(
        MessengerCommandExecutor::request_buddy(&players, &messenger_dao, &messenger, "missing",)
            .unwrap(),
        MessengerCommandOutcome::Ignored
    );
    assert_eq!(
        MessengerCommandExecutor::request_buddy(&players, &messenger_dao, &messenger, "alice",)
            .unwrap(),
        MessengerCommandOutcome::Ignored
    );
    assert_eq!(
        MessengerCommandExecutor::request_buddy(&players, &messenger_dao, &messenger, "bob",)
            .unwrap(),
        MessengerCommandOutcome::Ignored
    );
    assert!(!messenger_dao.request_exists(1, 2).unwrap());
}

#[test]
fn accepts_and_declines_existing_requests() {
    let players = player_dao();
    let messenger_dao = InMemoryMessengerDao::new();
    messenger_dao.new_request(2, 1).unwrap();
    messenger_dao.new_request(3, 1).unwrap();
    let mut messenger = Messenger::new(1);
    messenger.load(Vec::new(), vec![friend(2, "bob"), friend(3, "carol")]);

    let accepted =
        MessengerCommandExecutor::accept_buddy(&players, &messenger_dao, &messenger, "bob")
            .unwrap();
    let declined =
        MessengerCommandExecutor::decline_buddy(&players, &messenger_dao, &messenger, "carol")
            .unwrap();

    assert_eq!(
        accepted,
        MessengerCommandOutcome::BuddyAccepted { user_id: 2 }
    );
    assert_eq!(
        declined,
        MessengerCommandOutcome::BuddyDeclined { user_id: 3 }
    );
    assert!(messenger_dao
        .friends(1)
        .unwrap()
        .iter()
        .any(|user| user.user_id() == 2));
    assert!(!messenger_dao.request_exists(2, 1).unwrap());
    assert!(!messenger_dao.request_exists(3, 1).unwrap());
}

#[test]
fn removes_existing_friend_only() {
    let players = player_dao();
    let messenger_dao = InMemoryMessengerDao::new();
    messenger_dao.new_friend(1, 2).unwrap();
    let mut messenger = Messenger::new(1);
    messenger.load(vec![friend(2, "bob")], Vec::new());

    let outcome =
        MessengerCommandExecutor::remove_buddy(&players, &messenger_dao, &messenger, "bob")
            .unwrap();

    assert_eq!(
        outcome,
        MessengerCommandOutcome::BuddyRemoved { user_id: 2 }
    );
    assert!(messenger_dao.friends(1).unwrap().is_empty());
    assert_eq!(
        MessengerCommandExecutor::remove_buddy(&players, &messenger_dao, &messenger, "carol")
            .unwrap(),
        MessengerCommandOutcome::Ignored
    );
}

#[test]
fn sends_messages_only_to_current_friends() {
    let messenger_dao = InMemoryMessengerDao::new().with_current_time(1234);
    let mut messenger = Messenger::new(1);
    messenger.load(vec![friend(2, "bob")], Vec::new());

    let outcome =
        MessengerCommandExecutor::send_message(&messenger_dao, &messenger, &[2, 3], "hello")
            .unwrap();

    assert_eq!(
        outcome,
        MessengerCommandOutcome::MessagesSent {
            message_ids: vec![1]
        }
    );
    let unread = messenger_dao.unread_messages(2).unwrap();
    assert_eq!(unread.len(), 1);
    assert_eq!(unread[0].message(), "hello");
    assert!(messenger_dao.unread_messages(3).unwrap().is_empty());
}

#[test]
fn marks_read_only_for_current_users_unread_message() {
    let messenger_dao = InMemoryMessengerDao::new();
    let message_id = messenger_dao.new_message(1, 2, "hello").unwrap();
    let other_message_id = messenger_dao.new_message(1, 3, "other").unwrap();

    let ignored = MessengerCommandExecutor::mark_read(&messenger_dao, 2, other_message_id).unwrap();
    let outcome = MessengerCommandExecutor::mark_read(&messenger_dao, 2, message_id).unwrap();

    assert_eq!(ignored, MessengerCommandOutcome::Ignored);
    assert_eq!(
        outcome,
        MessengerCommandOutcome::MessageMarkedRead { message_id }
    );
    assert!(messenger_dao.unread_messages(2).unwrap().is_empty());
    assert_eq!(messenger_dao.unread_messages(3).unwrap().len(), 1);
}
