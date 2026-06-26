use super::*;
use crate::dao::in_memory::InMemoryPlayerDao;
use crate::dao::{CreatePlayer, PlayerDao};
use crate::game::player::{PlayerEffect, PlayerSession};
use crate::messages::OutgoingMessage;

const MAIN_PORT: i32 = 30001;

fn create_player(username: &str, password: &str) -> CreatePlayer {
    CreatePlayer::new(
        username,
        password,
        format!("{username}@example.test"),
        "",
        "hd=180-1.ch=215-91.lg=695-91",
        0,
        "M",
        "01-01-2000",
    )
}

fn request<'a>(username: &'a str, password: &'a str) -> PlayerLoginRequest<'a> {
    PlayerLoginRequest::new(username, password, false, 2, MAIN_PORT, MAIN_PORT)
}

#[test]
fn missing_user_maps_to_login_error_packet() {
    let dao = InMemoryPlayerDao::new();
    let manager = PlayerManager::new(vec![]);

    let outcome = PlayerLoginExecutor::login(&dao, &manager, request("unknown", "secret")).unwrap();

    let mut packet = outcome.login_error().unwrap().compose();
    assert_eq!(packet.get(), "#ERROR Login incorrect##");
    assert!(outcome.details().is_none());
}

#[test]
fn wrong_password_maps_to_login_error_packet() {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&create_player("alice", "correct"))
        .unwrap();
    let manager = PlayerManager::new(vec![]);

    let outcome = PlayerLoginExecutor::login(&dao, &manager, request("alice", "wrong")).unwrap();

    let mut packet = outcome.login_error().unwrap().compose();
    assert_eq!(packet.get(), "#ERROR Login incorrect##");
    assert!(outcome.effects().is_empty());
}

#[test]
fn correct_password_authenticates_with_raw_request_password() {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&create_player("alice", "secret"))
        .unwrap();
    let manager = PlayerManager::new(vec![]);

    let outcome = PlayerLoginExecutor::login(&dao, &manager, request("alice", "secret")).unwrap();
    let details = outcome.details().unwrap();

    assert!(details.is_authenticated());
    assert_eq!(details.username(), "alice");
    assert_eq!(details.password(), "secret");
    assert_eq!(
        outcome.effects(),
        &[PlayerEffect::UpdateLastLogin {
            user_id: details.id()
        }]
    );
    assert!(outcome.login_error().is_none());
}

#[test]
fn duplicate_on_same_port_produces_close_connection_effect() {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&create_player("alice", "secret"))
        .unwrap();
    let details = dao.details_by_username("alice").unwrap().unwrap();
    let user_id = details.id();
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(1, MAIN_PORT, details));

    let outcome = PlayerLoginExecutor::login(&dao, &manager, request("alice", "secret")).unwrap();

    assert_eq!(
        outcome.effects(),
        &[
            PlayerEffect::CloseConnection { connection_id: 1 },
            PlayerEffect::UpdateLastLogin { user_id },
        ]
    );
}

#[test]
fn room_login_keeps_java_public_room_lookup_id() {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&create_player("alice", "secret"))
        .unwrap();
    let manager = PlayerManager::new(vec![]);

    let outcome = PlayerLoginExecutor::login(
        &dao,
        &manager,
        PlayerLoginRequest::new("alice", "secret", true, 2, 30045, MAIN_PORT),
    )
    .unwrap();

    assert_eq!(outcome.public_room_lookup_id(), Some(44));
}
