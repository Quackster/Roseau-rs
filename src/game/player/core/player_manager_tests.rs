use crate::game::player::{Permission, PlayerDetails, PlayerManager, PlayerSession};

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

#[test]
fn finds_sessions_by_id_name_port_and_connection() {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(10, 30000, details(1, "Alice")));
    manager.insert(PlayerSession::new(11, 30001, details(1, "Alice")));
    manager.insert(PlayerSession::new(12, 30000, details(2, "Bob")));

    assert_eq!(manager.get_by_id(1).unwrap().details().username(), "Alice");
    assert_eq!(manager.get_by_name("alice").unwrap().details().id(), 1);
    assert_eq!(
        manager.get_by_id_on_port(1, 30001).unwrap().connection_id(),
        11
    );
    assert_eq!(
        manager
            .get_player_different_connection(1, 10)
            .unwrap()
            .connection_id(),
        11
    );
    assert_eq!(manager.main_server_players(30000).len(), 2);
}

#[test]
fn syncs_tickets_for_matching_user_id() {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(10, 30000, details(1, "Alice")));
    manager.insert(PlayerSession::new(11, 30001, details(1, "Alice")));
    manager.insert(PlayerSession::new(12, 30000, details(2, "Bob")));

    manager.sync_player_tickets(1, 12);

    assert_eq!(manager.players().get(&10).unwrap().details().tickets(), 12);
    assert_eq!(manager.players().get(&11).unwrap().details().tickets(), 12);
    assert_eq!(manager.players().get(&12).unwrap().details().tickets(), 0);
}

#[test]
fn syncs_credits_for_matching_user_id() {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(10, 30000, details(1, "Alice")));
    manager.insert(PlayerSession::new(11, 30001, details(1, "Alice")));
    manager.insert(PlayerSession::new(12, 30000, details(2, "Bob")));

    manager.sync_player_credits(1, 125);

    assert_eq!(manager.players().get(&10).unwrap().details().credits(), 125);
    assert_eq!(manager.players().get(&11).unwrap().details().credits(), 125);
    assert_eq!(manager.players().get(&12).unwrap().details().credits(), 0);
}

#[test]
fn detects_duplicate_authenticated_user_on_different_connection() {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(10, 30000, details(1, "Alice")));

    let duplicate = PlayerSession::new(11, 30001, details(1, "Alice"));
    let same_connection = PlayerSession::new(10, 30001, details(1, "Alice"));

    assert!(manager.check_for_duplicates(&duplicate));
    assert!(!manager.check_for_duplicates(&same_connection));
}

#[test]
fn checks_inheritable_and_exact_rank_permissions() {
    let manager = PlayerManager::new(vec![
        Permission::new("room_admin", true, 5),
        Permission::new("exact_rank", false, 7),
    ]);

    assert!(manager.has_permission(6, "room_admin"));
    assert!(manager.has_permission(7, "exact_rank"));
    assert!(!manager.has_permission(6, "exact_rank"));
    assert!(!manager.has_permission(4, "room_admin"));
}
