use bcrypt::BcryptError;

use crate::dao::mysql::{mapper, PlayerQueries, SqlExecutionPlan};
use crate::dao::{CreatePlayer, DaoError};
use crate::game::player::{PasswordHasher, PlayerDetails};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerPasswordQueries {
    hasher: PasswordHasher,
}

impl PlayerPasswordQueries {
    pub const fn new(hasher: PasswordHasher) -> Self {
        Self { hasher }
    }

    pub const fn java_compatible() -> Self {
        Self::new(PasswordHasher::java_compatible())
    }

    pub const fn hasher(&self) -> PasswordHasher {
        self.hasher
    }

    pub fn login_lookup_plan(username: &str) -> SqlExecutionPlan {
        PlayerQueries::login(username).read_plan()
    }

    pub fn verified_login_details(
        &self,
        row: &crate::dao::mysql::entity::UserRow,
        password: &str,
    ) -> Result<Option<PlayerDetails>, BcryptError> {
        if self.hasher.verify_password(password, &row.password)? {
            Ok(Some(mapper::player_details_from_row(row)))
        } else {
            Ok(None)
        }
    }

    pub fn create_player_plan(
        &self,
        player: &CreatePlayer,
        default_credits: i32,
        messenger_greeting: &str,
        now: i64,
    ) -> Result<SqlExecutionPlan, BcryptError> {
        let password_hash = self.hasher.hash_password(&player.password)?;
        Ok(PlayerQueries::create_player(
            player,
            &password_hash,
            default_credits,
            messenger_greeting,
            now,
        )
        .insert_returning_id_plan())
    }

    pub fn update_player_plan(
        &self,
        details: &PlayerDetails,
    ) -> Result<SqlExecutionPlan, BcryptError> {
        let password_hash = self.hasher.hash_password(details.password())?;
        Ok(PlayerQueries::update_player(details, &password_hash).execute_plan())
    }

    pub fn login_error(error: BcryptError) -> DaoError {
        DaoError::new(format!("Failed to verify player password: {error}"))
    }

    pub fn hash_error(error: BcryptError) -> DaoError {
        DaoError::new(format!("Failed to hash player password: {error}"))
    }
}

impl Default for PlayerPasswordQueries {
    fn default() -> Self {
        Self::java_compatible()
    }
}
