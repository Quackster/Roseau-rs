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
