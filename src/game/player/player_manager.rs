use std::collections::HashMap;

use crate::game::player::{Permission, PlayerDetails};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerSession {
    connection_id: i32,
    server_port: i32,
    details: PlayerDetails,
}

impl PlayerSession {
    pub fn new(connection_id: i32, server_port: i32, details: PlayerDetails) -> Self {
        Self {
            connection_id,
            server_port,
            details,
        }
    }

    pub fn connection_id(&self) -> i32 {
        self.connection_id
    }

    pub fn server_port(&self) -> i32 {
        self.server_port
    }

    pub fn details(&self) -> &PlayerDetails {
        &self.details
    }

    pub fn details_mut(&mut self) -> &mut PlayerDetails {
        &mut self.details
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerManager {
    players: HashMap<i32, PlayerSession>,
    permissions: Vec<Permission>,
}

impl PlayerManager {
    pub fn new(permissions: Vec<Permission>) -> Self {
        Self {
            players: HashMap::new(),
            permissions,
        }
    }

    pub fn insert(&mut self, player: PlayerSession) -> Option<PlayerSession> {
        self.players.insert(player.connection_id(), player)
    }

    pub fn remove(&mut self, connection_id: i32) -> Option<PlayerSession> {
        self.players.remove(&connection_id)
    }

    pub fn get_by_id(&self, user_id: i32) -> Option<&PlayerSession> {
        self.players
            .values()
            .find(|session| session.details().id() == user_id)
    }

    pub fn get_by_id_on_port(&self, user_id: i32, server_port: i32) -> Option<&PlayerSession> {
        self.players.values().find(|session| {
            session.details().id() == user_id && session.server_port() == server_port
        })
    }

    pub fn get_by_name(&self, name: &str) -> Option<&PlayerSession> {
        self.players
            .values()
            .find(|session| session.details().username().eq_ignore_ascii_case(name))
    }

    pub fn get_private_room_player(
        &self,
        user_id: i32,
        private_server_port: i32,
    ) -> Option<&PlayerSession> {
        self.get_by_id_on_port(user_id, private_server_port)
    }

    pub fn get_private_room_player_by_name(
        &self,
        username: &str,
        private_server_port: i32,
    ) -> Option<&PlayerSession> {
        self.players.values().find(|session| {
            session.details().username().eq_ignore_ascii_case(username)
                && session.server_port() == private_server_port
        })
    }

    pub fn get_player_different_connection(
        &self,
        user_id: i32,
        connection_id: i32,
    ) -> Option<&PlayerSession> {
        self.players.values().find(|session| {
            session.details().id() == user_id && session.connection_id() != connection_id
        })
    }

    pub fn get_player_by_port_different_connection(
        &self,
        user_id: i32,
        server_port: i32,
        connection_id: i32,
    ) -> Option<&PlayerSession> {
        self.players.values().find(|session| {
            session.details().id() == user_id
                && session.server_port() == server_port
                && session.connection_id() != connection_id
        })
    }

    pub fn sync_player_tickets(&mut self, user_id: i32, tickets: i32) {
        for session in self
            .players
            .values_mut()
            .filter(|session| session.details().id() == user_id)
        {
            session.details_mut().set_tickets(tickets);
        }
    }

    pub fn sync_player_credits(&mut self, user_id: i32, credits: i32) {
        for session in self
            .players
            .values_mut()
            .filter(|session| session.details().id() == user_id)
        {
            session.details_mut().set_credits(credits);
        }
    }

    pub fn check_for_duplicates(&self, player: &PlayerSession) -> bool {
        if player.connection_id() == -1 || player.details().id() == -1 {
            return false;
        }

        self.players.values().any(|session| {
            session.connection_id() != -1
                && session.details().id() == player.details().id()
                && session.connection_id() != player.connection_id()
        })
    }

    pub fn main_server_players(&self, server_port: i32) -> Vec<&PlayerSession> {
        self.players
            .values()
            .filter(|session| session.server_port() == server_port)
            .collect()
    }

    pub fn has_permission(&self, rank: i32, permission_name: &str) -> bool {
        self.permissions.iter().any(|permission| {
            permission.permission() == permission_name
                && ((permission.is_inheritable() && rank >= permission.rank())
                    || (!permission.is_inheritable() && rank == permission.rank()))
        })
    }

    pub fn players(&self) -> &HashMap<i32, PlayerSession> {
        &self.players
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
