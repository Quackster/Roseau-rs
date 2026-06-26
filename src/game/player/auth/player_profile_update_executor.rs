use crate::dao::{DaoError, PlayerDao};
use crate::game::player::{PasswordAction, PlayerDetails};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerProfileUpdateExecutor;

impl PlayerProfileUpdateExecutor {
    pub fn update_profile(
        player_dao: &dyn PlayerDao,
        current: &PlayerDetails,
        action: &PasswordAction,
    ) -> Result<PlayerProfileUpdateOutcome, DaoError> {
        let Some(updated) = action.updated_profile_details(current) else {
            return Ok(PlayerProfileUpdateOutcome::Ignored);
        };

        player_dao.update_player(&updated)?;
        Ok(PlayerProfileUpdateOutcome::Updated(updated))
    }

    pub fn update_pool_figure(
        player_dao: &dyn PlayerDao,
        current: &PlayerDetails,
        pool_figure: &str,
    ) -> Result<PlayerProfileUpdateOutcome, DaoError> {
        let mut updated = current.clone();
        updated.set_pool_figure(pool_figure);
        player_dao.update_player(&updated)?;
        Ok(PlayerProfileUpdateOutcome::Updated(updated))
    }

    pub fn update_personal_greeting(
        player_dao: &dyn PlayerDao,
        current: &PlayerDetails,
        personal_greeting: &str,
    ) -> Result<PlayerProfileUpdateOutcome, DaoError> {
        let mut updated = current.clone();
        updated.set_personal_greeting(personal_greeting);
        player_dao.update_player(&updated)?;
        Ok(PlayerProfileUpdateOutcome::Updated(updated))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerProfileUpdateOutcome {
    Updated(PlayerDetails),
    Ignored,
}

impl PlayerProfileUpdateOutcome {
    pub fn details(&self) -> Option<&PlayerDetails> {
        match self {
            Self::Updated(details) => Some(details),
            Self::Ignored => None,
        }
    }
}

#[cfg(test)]
#[path = "player_profile_update_executor_tests.rs"]
mod tests;
