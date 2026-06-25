use crate::dao::{CreatePlayer, DaoError, PlayerDao};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerRegistrationExecutor;

impl PlayerRegistrationExecutor {
    pub fn register(
        player_dao: &dyn PlayerDao,
        request: PlayerRegistrationRequest<'_>,
    ) -> Result<PlayerRegistrationOutcome, DaoError> {
        if player_dao.is_name_taken(request.username)? {
            return Ok(PlayerRegistrationOutcome::NameTaken);
        }

        let player = CreatePlayer::new(
            request.username,
            request.password,
            request.email,
            request.mission,
            request.figure,
            request.default_credits,
            request.sex,
            request.birthday,
        );
        player_dao.create_player(&player)?;
        Ok(PlayerRegistrationOutcome::Created)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerRegistrationRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub mission: &'a str,
    pub figure: &'a str,
    pub sex: &'a str,
    pub birthday: &'a str,
    pub default_credits: i32,
}

impl<'a> PlayerRegistrationRequest<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        username: &'a str,
        password: &'a str,
        email: &'a str,
        mission: &'a str,
        figure: &'a str,
        sex: &'a str,
        birthday: &'a str,
        default_credits: i32,
    ) -> Self {
        Self {
            username,
            password,
            email,
            mission,
            figure,
            sex,
            birthday,
            default_credits,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerRegistrationOutcome {
    Created,
    NameTaken,
}
