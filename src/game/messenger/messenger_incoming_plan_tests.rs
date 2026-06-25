use super::messenger_incoming_plan::*;
use crate::dao::in_memory::{InMemoryMessengerDao, InMemoryPlayerDao};
use crate::dao::{CreatePlayer, MessengerDao, PlayerDao};
use crate::game::messenger::MessengerFriend;

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
fn plans_buddy_request_from_incoming_effect() {
    let players = player_dao();
    let messenger_dao = InMemoryMessengerDao::new();
    let messenger = Messenger::new(1);

    let outcomes = MessengerIncomingPlan::plan(
        &IncomingExecutionEffect::RequestBuddy {
            username: "bob".to_owned(),
        },
        &players,
        &messenger_dao,
        &messenger,
    )
    .unwrap();

    assert_eq!(
        outcomes,
        vec![MessengerCommandOutcome::BuddyRequestCreated { to_id: 2 }]
    );
    assert!(messenger_dao.request_exists(1, 2).unwrap());
}

#[test]
fn plans_accept_decline_remove_and_message_effects_in_order() {
    let players = player_dao();
    let messenger_dao = InMemoryMessengerDao::new();
    messenger_dao.new_request(2, 1).unwrap();
    messenger_dao.new_request(3, 1).unwrap();
    messenger_dao.new_friend(1, 2).unwrap();
    let mut messenger = Messenger::new(1);
    messenger.load(
        vec![friend(2, "bob")],
        vec![friend(2, "bob"), friend(3, "carol")],
    );

    let outcomes = MessengerIncomingPlan::plan_all(
        &[
            IncomingExecutionEffect::AcceptBuddy {
                username: "bob".to_owned(),
            },
            IncomingExecutionEffect::DeclineBuddy {
                username: "carol".to_owned(),
            },
            IncomingExecutionEffect::SendMessengerMessage {
                receiver_ids: vec![2, 3],
                message: "hello".to_owned(),
            },
            IncomingExecutionEffect::RemoveBuddy {
                username: "bob".to_owned(),
            },
        ],
        &players,
        &messenger_dao,
        &messenger,
    )
    .unwrap();

    assert_eq!(
        outcomes,
        vec![
            MessengerCommandOutcome::BuddyAccepted { user_id: 2 },
            MessengerCommandOutcome::BuddyDeclined { user_id: 3 },
            MessengerCommandOutcome::MessagesSent {
                message_ids: vec![1]
            },
            MessengerCommandOutcome::BuddyRemoved { user_id: 2 },
        ]
    );
}

#[test]
fn plans_mark_read_for_current_messenger_user() {
    let players = player_dao();
    let messenger_dao = InMemoryMessengerDao::new();
    let message_id = messenger_dao.new_message(2, 1, "hello").unwrap();
    let messenger = Messenger::new(1);

    let outcomes = MessengerIncomingPlan::plan(
        &IncomingExecutionEffect::MarkMessengerMessageRead { message_id },
        &players,
        &messenger_dao,
        &messenger,
    )
    .unwrap();

    assert_eq!(
        outcomes,
        vec![MessengerCommandOutcome::MessageMarkedRead { message_id }]
    );
    assert!(messenger_dao.unread_messages(1).unwrap().is_empty());
}

#[test]
fn ignores_unrelated_incoming_effects() {
    assert!(MessengerIncomingPlan::plan(
        &IncomingExecutionEffect::GoAway,
        &player_dao(),
        &InMemoryMessengerDao::new(),
        &Messenger::new(1),
    )
    .unwrap()
    .is_empty());
}
