use super::*;

#[test]
fn tracks_session_lifecycle() {
    let mut manager = SessionManager::new();

    assert_eq!(manager.add_session(7).connection_id(), 7);
    assert!(manager.has_session(7));
    assert_eq!(manager.get_session(7).unwrap().player_id(), None);

    manager.add_player_session(7, 42);

    assert_eq!(manager.get_session(7).unwrap().player_id(), Some(42));
    assert_eq!(manager.remove_session(7).unwrap().connection_id(), 7);
    assert!(!manager.has_session(7));
}

#[test]
fn plans_connection_session_creation_side_effects() {
    assert_eq!(
        SessionManager::add_session_effects(11, "/127.0.0.1:37120"),
        Some(vec![
            SessionLifecycleEffect::CreatePlayerNetwork {
                connection_id: 11,
                server_port: 37120,
            },
            SessionLifecycleEffect::AttachPlayer { connection_id: 11 },
            SessionLifecycleEffect::RegisterPlayer { connection_id: 11 },
            SessionLifecycleEffect::StoreSession { connection_id: 11 },
        ])
    );
}

#[test]
fn rejects_session_creation_when_local_port_is_missing() {
    assert_eq!(SessionManager::add_session_effects(11, "/127.0.0.1"), None);
}

#[test]
fn plans_connection_session_removal_side_effects() {
    assert_eq!(
        SessionManager::remove_session_effects(11),
        vec![
            SessionLifecycleEffect::RemovePlayer { connection_id: 11 },
            SessionLifecycleEffect::RemoveSession { connection_id: 11 },
        ]
    );
}
