use std::collections::HashMap;

use crate::game::player::{Permission, PlayerDetails};
use crate::game::room::entity::RoomUser;

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerSession {
    connection_id: i32,
    server_port: i32,
    details: PlayerDetails,
    room_user: Option<RoomUser>,
    pending_private_room_id: Option<i32>,
}

impl PlayerSession {
    pub fn new(connection_id: i32, server_port: i32, details: PlayerDetails) -> Self {
        Self {
            connection_id,
            server_port,
            details,
            room_user: None,
            pending_private_room_id: None,
        }
    }

    pub fn connection_id(&self) -> i32 {
        self.connection_id
    }

    pub fn server_port(&self) -> i32 {
        self.server_port
    }

    pub fn set_server_port(&mut self, server_port: i32) {
        self.server_port = server_port;
    }

    pub fn details(&self) -> &PlayerDetails {
        &self.details
    }

    pub fn details_mut(&mut self) -> &mut PlayerDetails {
        &mut self.details
    }

    pub fn room_user(&self) -> Option<&RoomUser> {
        self.room_user.as_ref()
    }

    pub fn room_user_mut(&mut self) -> Option<&mut RoomUser> {
        self.room_user.as_mut()
    }

    pub fn set_room_user(&mut self, room_user: RoomUser) {
        self.room_user = Some(room_user);
    }

    pub fn clear_room_user(&mut self) {
        self.room_user = None;
    }

    pub fn pending_private_room_id(&self) -> Option<i32> {
        self.pending_private_room_id
    }

    pub fn set_pending_private_room_id(&mut self, room_id: i32) {
        self.pending_private_room_id = Some(room_id);
    }

    pub fn clear_pending_private_room_id(&mut self) {
        self.pending_private_room_id = None;
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn get_mut(&mut self, connection_id: i32) -> Option<&mut PlayerSession> {
        self.players.get_mut(&connection_id)
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

    pub fn pending_private_room_id_for_user(&self, user_id: i32) -> Option<i32> {
        self.players
            .values()
            .find(|session| session.details().id() == user_id)
            .and_then(PlayerSession::pending_private_room_id)
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

    pub fn sync_player_details(&mut self, details: &PlayerDetails) {
        for session in self
            .players
            .values_mut()
            .filter(|session| session.details().id() == details.id())
        {
            *session.details_mut() = details.clone();
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

    pub fn set_permissions(&mut self, permissions: impl Into<Vec<Permission>>) {
        self.permissions = permissions.into();
    }
}
