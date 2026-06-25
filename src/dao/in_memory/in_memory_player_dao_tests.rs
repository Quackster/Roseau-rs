use super::in_memory_player_dao::*;

fn create_player(username: &str, password: &str) -> CreatePlayer {
    CreatePlayer::new(
        username,
        password,
        format!("{username}@example.test"),
        "hello",
        "hd-100",
        50,
        "F",
        "1990-01-01",
    )
}

#[test]
fn creates_and_reads_players_by_name_and_id() {
    let dao = InMemoryPlayerDao::new();

    dao.create_player(&create_player("alice", "secret"))
        .unwrap();
    dao.create_player(&create_player("bob", "door")).unwrap();

    assert_eq!(dao.len(), 2);
    assert!(dao.is_name_taken("alice").unwrap());
    assert_eq!(dao.id_by_username("bob").unwrap(), Some(2));
    assert_eq!(dao.details_by_id(1).unwrap().unwrap().username(), "alice");
}

#[test]
fn logs_in_only_with_matching_password() {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&create_player("alice", "secret"))
        .unwrap();

    assert!(dao.login("alice", "wrong").unwrap().is_none());

    let login = dao.login("alice", "secret").unwrap().unwrap();
    assert!(login.authenticated);
    assert_eq!(login.details.username(), "alice");
}

#[test]
fn updates_player_and_last_login_time() {
    let dao = InMemoryPlayerDao::new().with_last_login_time(12345);
    dao.create_player(&create_player("alice", "secret"))
        .unwrap();

    let mut details = dao.details_by_username("alice").unwrap().unwrap();
    details.set_mission("changed");
    dao.update_player(&details).unwrap();
    dao.update_last_login(&details).unwrap();

    let updated = dao.details_by_username("alice").unwrap().unwrap();
    assert_eq!(updated.mission(), "changed");
    assert_eq!(updated.last_online(), 12345);
}
