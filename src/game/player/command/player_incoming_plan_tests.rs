use super::*;
use crate::dao::in_memory::InMemoryPlayerDao;
use crate::dao::{CreatePlayer, PlayerDao};
use crate::messages::OutgoingMessage;

fn create_player(username: &str) -> CreatePlayer {
    CreatePlayer::new(
        username,
        "secret",
        format!("{username}@example.test"),
        format!("hello {username}"),
        "hd=100",
        50,
        "F",
        "1990-01-01",
    )
}

fn player_dao() -> InMemoryPlayerDao {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&create_player("alice")).unwrap();
    dao.create_player(&create_player("bob")).unwrap();
    dao
}

fn current_player() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        7,
        "alice",
        "mission",
        "hd=100",
        "",
        "alice@example.test",
        1,
        50,
        "F",
        "UK",
        "",
        "1990-01-01",
        123,
        "hello",
        9,
    );
    details
}

#[test]
fn plans_current_player_info_and_tickets() {
    let outcomes = PlayerIncomingPlan::plan_all(
        &[
            IncomingExecutionEffect::RetrieveUserInfo,
            IncomingExecutionEffect::SendTickets,
        ],
        &player_dao(),
        &current_player(),
        "last",
        "Hotel View",
    )
    .unwrap();

    assert_eq!(outcomes.len(), 2);
    let PlayerIncomingOutcome::Command(info) = &outcomes[0] else {
        panic!("expected player command outcome");
    };
    let PlayerIncomingOutcome::Command(tickets) = &outcomes[1] else {
        panic!("expected ticket command outcome");
    };
    assert_eq!(info.user_object().unwrap().compose().header(), "USEROBJECT");
    assert_eq!(
        tickets.ph_tickets().unwrap().compose().get(),
        "#PH_TICKETS 9##"
    );
}

#[test]
fn plans_found_user_lookup_from_player_dao() {
    let outcomes = PlayerIncomingPlan::plan(
        &IncomingExecutionEffect::FindUser {
            username: "bob".to_owned(),
        },
        &player_dao(),
        &current_player(),
        "now",
        "On Hotel View",
    )
    .unwrap();

    assert_eq!(outcomes.len(), 1);
    let PlayerIncomingOutcome::FindUser(outcome) = &outcomes[0] else {
        panic!("expected find-user outcome");
    };
    assert_eq!(
        outcome.member_info().unwrap().compose().get(),
        "#MEMBERINFO \rbob\r\rnow\rOn Hotel View\rhd=100##"
    );
}

#[test]
fn missing_or_empty_find_user_emits_missing_outcome() {
    for username in ["", "missing"] {
        let outcomes = PlayerIncomingPlan::plan(
            &IncomingExecutionEffect::FindUser {
                username: username.to_owned(),
            },
            &player_dao(),
            &current_player(),
            "now",
            "On Hotel View",
        )
        .unwrap();

        assert_eq!(
            outcomes,
            vec![PlayerIncomingOutcome::FindUser(FindUserOutcome::Missing)]
        );
    }
}

#[test]
fn ignores_unrelated_incoming_effects() {
    assert!(PlayerIncomingPlan::plan(
        &IncomingExecutionEffect::GoAway,
        &player_dao(),
        &current_player(),
        "now",
        "On Hotel View",
    )
    .unwrap()
    .is_empty());
}
