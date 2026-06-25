use std::collections::HashMap;

use crate::server::{PlayerNetworkPlan, Session, SessionLifecycleEffect};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SessionManager {
    sessions: HashMap<i32, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_session(&mut self, connection_id: i32) -> &Session {
        self.sessions
            .entry(connection_id)
            .or_insert_with(|| Session::new(connection_id))
    }

    pub fn add_player_session(&mut self, connection_id: i32, player_id: i32) -> &Session {
        self.sessions
            .entry(connection_id)
            .and_modify(|session| session.set_player_id(player_id))
            .or_insert_with(|| Session::new(connection_id).with_player_id(player_id))
    }

    pub fn remove_session(&mut self, connection_id: i32) -> Option<Session> {
        self.sessions.remove(&connection_id)
    }

    pub fn has_session(&self, connection_id: i32) -> bool {
        self.sessions.contains_key(&connection_id)
    }

    pub fn get_session(&self, connection_id: i32) -> Option<&Session> {
        self.sessions.get(&connection_id)
    }

    pub fn sessions(&self) -> &HashMap<i32, Session> {
        &self.sessions
    }

    pub fn add_session_effects(
        connection_id: i32,
        local_address: &str,
    ) -> Option<Vec<SessionLifecycleEffect>> {
        let network_plan = PlayerNetworkPlan::from_local_address(connection_id, local_address)?;

        Some(vec![
            SessionLifecycleEffect::CreatePlayerNetwork {
                connection_id,
                server_port: network_plan.server_port(),
            },
            SessionLifecycleEffect::AttachPlayer { connection_id },
            SessionLifecycleEffect::RegisterPlayer { connection_id },
            SessionLifecycleEffect::StoreSession { connection_id },
        ])
    }

    pub fn remove_session_effects(connection_id: i32) -> Vec<SessionLifecycleEffect> {
        vec![
            SessionLifecycleEffect::RemovePlayer { connection_id },
            SessionLifecycleEffect::RemoveSession { connection_id },
        ]
    }
}

#[cfg(test)]
mod tests {
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
}
