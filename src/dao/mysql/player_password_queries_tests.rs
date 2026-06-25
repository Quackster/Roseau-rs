use super::*;
use crate::dao::mysql::entity::UserRow;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

fn planner() -> PlayerPasswordQueries {
    PlayerPasswordQueries::new(PasswordHasher::new(4))
}

fn create_player() -> CreatePlayer {
    CreatePlayer::new(
        "alice",
        "secret",
        "alice@example.test",
        "hello",
        "hd-100",
        1,
        "F",
        "1990-01-01",
    )
}

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        7,
        "alice",
        "hello",
        "hd-100",
        "pool",
        "alice@example.test",
        1,
        100,
        "F",
        "UK",
        "",
        "1990-01-01",
        1234,
        "welcome",
        2,
    );
    details.set_password("changed");
    details
}

#[test]
fn builds_login_lookup_read_plan() {
    let plan = PlayerPasswordQueries::login_lookup_plan("alice");

    assert_eq!(plan.kind(), SqlExecutionKind::ReadRows);
    assert_eq!(plan.sql(), "SELECT * FROM users WHERE username = ? LIMIT 1");
    assert_eq!(plan.parameters(), &[SqlParameter::Text("alice".to_owned())]);
}

#[test]
fn verifies_login_row_before_mapping_details() {
    let hasher = PasswordHasher::new(4);
    let password_hash = hasher.hash_password("secret").unwrap();
    let row = UserRow::new(
        7,
        "alice",
        password_hash,
        1,
        "hello",
        "hd-100",
        "pool",
        "alice@example.test",
        100,
        "F",
        "UK",
        "",
        "1990-01-01",
        1000,
        2000,
        "welcome",
        2,
    );
    let planner = PlayerPasswordQueries::new(hasher);

    let details = planner.verified_login_details(&row, "secret").unwrap();
    let rejected = planner.verified_login_details(&row, "wrong").unwrap();

    assert_eq!(details.unwrap().username(), "alice");
    assert!(rejected.is_none());
}

#[test]
fn hashes_registration_password_before_insert_plan() {
    let plan = planner()
        .create_player_plan(&create_player(), 100, "welcome", 1234)
        .unwrap();

    assert_eq!(plan.kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(
        plan.sql(),
        "INSERT INTO users (username, password, email, mission, figure, credits, sex, birthday, join_date, last_online, personal_greeting) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    );
    assert_eq!(plan.parameters()[0], SqlParameter::Text("alice".to_owned()));
    assert!(
        matches!(&plan.parameters()[1], SqlParameter::Text(hash) if hash.starts_with("$2a$04$"))
    );
    assert_eq!(plan.parameters()[5], SqlParameter::Integer(100));
    assert_eq!(plan.parameters()[8], SqlParameter::Long(1234));
    assert_eq!(
        plan.parameters()[10],
        SqlParameter::Text("welcome".to_owned())
    );
}

#[test]
fn hashes_profile_password_before_update_plan() {
    let plan = planner().update_player_plan(&details()).unwrap();

    assert_eq!(plan.kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plan.sql(),
        "UPDATE users SET password = ?, figure = ?, credits = ?, mission = ?, pool_figure = ?, sex = ?, email = ?, personal_greeting = ?, tickets = ? WHERE id = ?"
    );
    assert!(
        matches!(&plan.parameters()[0], SqlParameter::Text(hash) if hash.starts_with("$2a$04$"))
    );
    assert_eq!(
        plan.parameters()[1],
        SqlParameter::Text("hd-100".to_owned())
    );
    assert_eq!(plan.parameters()[9], SqlParameter::Integer(7));
}
