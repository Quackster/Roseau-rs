use crate::dao::mysql::{PlayerQueries, SqlParameter};
use crate::dao::CreatePlayer;
use crate::game::player::PlayerDetails;

fn create_player() -> CreatePlayer {
    CreatePlayer::new(
        "alice",
        "plain",
        "alice@example.test",
        "hello",
        "hd-100",
        999,
        "F",
        "1990-01-01",
    )
}

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        7,
        "alice",
        "new mission",
        "hd-200",
        "pool",
        "alice@example.test",
        2,
        55,
        "F",
        "UK",
        "ADM",
        "1990-01-01",
        123,
        "welcome",
        4,
    );
    details
}

#[test]
fn builds_create_player_insert_with_runtime_defaults() {
    let query = PlayerQueries::create_player(&create_player(), "hash", 100, "hi", 1234);

    assert_eq!(
        query.sql(),
        "INSERT INTO users (username, password, email, mission, figure, credits, sex, birthday, join_date, last_online, personal_greeting) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    );
    assert_eq!(
        query.parameters(),
        &[
            SqlParameter::Text("alice".to_owned()),
            SqlParameter::Text("hash".to_owned()),
            SqlParameter::Text("alice@example.test".to_owned()),
            SqlParameter::Text("hello".to_owned()),
            SqlParameter::Text("hd-100".to_owned()),
            SqlParameter::Integer(100),
            SqlParameter::Text("F".to_owned()),
            SqlParameter::Text("1990-01-01".to_owned()),
            SqlParameter::Long(1234),
            SqlParameter::Long(1234),
            SqlParameter::Text("hi".to_owned()),
        ]
    );
}

#[test]
fn builds_user_lookup_queries() {
    assert_eq!(
        PlayerQueries::details_by_id(7).sql(),
        "SELECT * FROM users WHERE id = ? LIMIT 1"
    );
    assert_eq!(
        PlayerQueries::details_by_username("alice").sql(),
        "SELECT * FROM users WHERE username = ? LIMIT 1"
    );
    assert_eq!(
        PlayerQueries::login("alice"),
        PlayerQueries::details_by_username("alice")
    );
    assert_eq!(
        PlayerQueries::id_by_username("alice").sql(),
        "SELECT id FROM users WHERE username = ? LIMIT 1"
    );
    assert_eq!(
        PlayerQueries::is_name_taken("alice").parameters(),
        &[SqlParameter::Text("alice".to_owned())]
    );
}

#[test]
fn builds_profile_and_last_login_updates() {
    let update = PlayerQueries::update_player(&details(), "hash2");
    let last_login = PlayerQueries::update_last_login(7, 999);

    assert_eq!(
        update.sql(),
        "UPDATE users SET password = ?, figure = ?, credits = ?, mission = ?, pool_figure = ?, sex = ?, email = ?, personal_greeting = ?, tickets = ? WHERE id = ?"
    );
    assert_eq!(
        update.parameters(),
        &[
            SqlParameter::Text("hash2".to_owned()),
            SqlParameter::Text("hd-200".to_owned()),
            SqlParameter::Integer(55),
            SqlParameter::Text("new mission".to_owned()),
            SqlParameter::Text("pool".to_owned()),
            SqlParameter::Text("F".to_owned()),
            SqlParameter::Text("alice@example.test".to_owned()),
            SqlParameter::Text("welcome".to_owned()),
            SqlParameter::Integer(4),
            SqlParameter::Integer(7),
        ]
    );
    assert_eq!(
        last_login.sql(),
        "UPDATE users SET last_online = ? WHERE id = ?"
    );
    assert_eq!(
        last_login.parameters(),
        &[SqlParameter::Long(999), SqlParameter::Integer(7)]
    );
}

#[test]
fn builds_credit_update_query() {
    let query = PlayerQueries::update_credits(7, 125);

    assert_eq!(query.sql(), "UPDATE users SET credits = ? WHERE id = ?");
    assert_eq!(
        query.parameters(),
        &[SqlParameter::Integer(125), SqlParameter::Integer(7)]
    );
}

#[test]
fn builds_ticket_update_query() {
    let query = PlayerQueries::update_tickets(7, 4);

    assert_eq!(query.sql(), "UPDATE users SET tickets = ? WHERE id = ?");
    assert_eq!(
        query.parameters(),
        &[SqlParameter::Integer(4), SqlParameter::Integer(7)]
    );
}

#[test]
fn builds_focused_profile_field_updates() {
    let greeting = PlayerQueries::update_personal_greeting(7, "hello");
    let pool_figure = PlayerQueries::update_pool_figure(7, "ph=1");

    assert_eq!(
        greeting.sql(),
        "UPDATE users SET personal_greeting = ? WHERE id = ?"
    );
    assert_eq!(
        greeting.parameters(),
        &[
            SqlParameter::Text("hello".to_owned()),
            SqlParameter::Integer(7)
        ]
    );
    assert_eq!(
        pool_figure.sql(),
        "UPDATE users SET pool_figure = ? WHERE id = ?"
    );
    assert_eq!(
        pool_figure.parameters(),
        &[
            SqlParameter::Text("ph=1".to_owned()),
            SqlParameter::Integer(7)
        ]
    );
}

#[test]
fn builds_permission_read_query() {
    assert_eq!(
        PlayerQueries::permissions().sql(),
        "SELECT * FROM users_permissions"
    );
    assert_eq!(
        (
            PlayerQueries::user_table(),
            PlayerQueries::permission_table()
        ),
        ("users", "users_permissions")
    );
}
