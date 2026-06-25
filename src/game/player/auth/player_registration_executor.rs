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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::InMemoryPlayerDao;
    use crate::dao::PlayerDao;

    fn request<'a>(username: &'a str, password: &'a str) -> PlayerRegistrationRequest<'a> {
        PlayerRegistrationRequest::new(
            username,
            password,
            "alice@example.test",
            "hello",
            "hd=180-1.ch=215-91.lg=695-91",
            "Male",
            "08.08.1997",
            100,
        )
    }

    #[test]
    fn creates_player_when_name_is_available() {
        let dao = InMemoryPlayerDao::new();

        let outcome =
            PlayerRegistrationExecutor::register(&dao, request("alice", "secret")).unwrap();

        let details = dao.details_by_username("alice").unwrap().unwrap();
        assert_eq!(outcome, PlayerRegistrationOutcome::Created);
        assert_eq!(details.password(), "secret");
        assert_eq!(details.email(), "alice@example.test");
        assert_eq!(details.mission(), "hello");
        assert_eq!(details.figure(), "hd=180-1.ch=215-91.lg=695-91");
        assert_eq!(details.sex(), "Male");
        assert_eq!(details.birthday(), "08.08.1997");
        assert_eq!(details.credits(), 100);
    }

    #[test]
    fn taken_name_does_not_overwrite_existing_player() {
        let dao = InMemoryPlayerDao::new();
        PlayerRegistrationExecutor::register(&dao, request("alice", "secret")).unwrap();

        let outcome =
            PlayerRegistrationExecutor::register(&dao, request("alice", "changed")).unwrap();

        let details = dao.details_by_username("alice").unwrap().unwrap();
        assert_eq!(outcome, PlayerRegistrationOutcome::NameTaken);
        assert_eq!(details.password(), "secret");
        assert_eq!(dao.len(), 1);
    }
}
