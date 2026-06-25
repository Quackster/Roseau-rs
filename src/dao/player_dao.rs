use crate::dao::DaoError;
use crate::game::player::{Permission, PlayerDetails};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePlayer {
    pub username: String,
    pub password: String,
    pub email: String,
    pub mission: String,
    pub figure: String,
    pub credits: i32,
    pub sex: String,
    pub birthday: String,
}

impl CreatePlayer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        username: impl Into<String>,
        password: impl Into<String>,
        email: impl Into<String>,
        mission: impl Into<String>,
        figure: impl Into<String>,
        credits: i32,
        sex: impl Into<String>,
        birthday: impl Into<String>,
    ) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            email: email.into(),
            mission: mission.into(),
            figure: figure.into(),
            credits,
            sex: sex.into(),
            birthday: birthday.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginResult {
    pub details: PlayerDetails,
    pub authenticated: bool,
}

impl LoginResult {
    pub fn new(details: PlayerDetails, authenticated: bool) -> Self {
        Self {
            details,
            authenticated,
        }
    }
}

pub trait PlayerDao {
    fn create_player(&self, player: &CreatePlayer) -> Result<(), DaoError>;
    fn details_by_id(&self, user_id: i32) -> Result<Option<PlayerDetails>, DaoError>;
    fn login(&self, username: &str, password: &str) -> Result<Option<LoginResult>, DaoError>;
    fn id_by_username(&self, username: &str) -> Result<Option<i32>, DaoError>;
    fn is_name_taken(&self, name: &str) -> Result<bool, DaoError>;
    fn update_player(&self, details: &PlayerDetails) -> Result<(), DaoError>;
    fn update_last_login(&self, details: &PlayerDetails) -> Result<(), DaoError>;
    fn permissions(&self) -> Result<Vec<Permission>, DaoError>;
    fn details_by_username(&self, username: &str) -> Result<Option<PlayerDetails>, DaoError>;
}
