use crate::dao::{DaoError, PlayerDao};
use crate::game::player::{PlayerLoginOutcome, PlayerManager};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerLoginExecutor;

impl PlayerLoginExecutor {
    pub fn login(
        player_dao: &dyn PlayerDao,
        player_manager: &PlayerManager,
        request: PlayerLoginRequest<'_>,
    ) -> Result<PlayerLoginOutcome, DaoError> {
        let Some(login_result) = player_dao.login(request.username, request.password)? else {
            return Ok(PlayerLoginOutcome::failed());
        };

        if !login_result.authenticated {
            return Ok(PlayerLoginOutcome::failed());
        }

        let duplicate_connection_id = player_manager
            .get_player_by_port_different_connection(
                login_result.details.id(),
                request.server_port,
                request.connection_id,
            )
            .map(|session| session.connection_id());

        Ok(PlayerLoginOutcome::authenticated(
            &login_result.details,
            request.password,
            request.room_login,
            request.server_port,
            request.base_server_port,
            duplicate_connection_id,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerLoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub room_login: bool,
    pub connection_id: i32,
    pub server_port: i32,
    pub base_server_port: i32,
}

impl<'a> PlayerLoginRequest<'a> {
    pub fn new(
        username: &'a str,
        password: &'a str,
        room_login: bool,
        connection_id: i32,
        server_port: i32,
        base_server_port: i32,
    ) -> Self {
        Self {
            username,
            password,
            room_login,
            connection_id,
            server_port,
            base_server_port,
        }
    }
}

#[cfg(test)]
#[path = "player_login_executor_tests.rs"]
mod tests;
