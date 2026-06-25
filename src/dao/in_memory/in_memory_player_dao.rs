use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use crate::dao::{CreatePlayer, DaoError, LoginResult, PlayerDao};
use crate::game::player::{Permission, PlayerDetails};

#[derive(Debug, Default)]
pub struct InMemoryPlayerDao {
    by_username: RefCell<HashMap<String, PlayerDetails>>,
    next_id: Cell<i32>,
    last_login_time: Cell<i64>,
}

impl InMemoryPlayerDao {
    pub fn new() -> Self {
        Self {
            by_username: RefCell::new(HashMap::new()),
            next_id: Cell::new(1),
            last_login_time: Cell::new(0),
        }
    }

    pub fn with_last_login_time(mut self, last_login_time: i64) -> Self {
        self.last_login_time = Cell::new(last_login_time);
        self
    }

    pub fn set_last_login_time(&self, last_login_time: i64) {
        self.last_login_time.set(last_login_time);
    }

    pub fn len(&self) -> usize {
        self.by_username.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_username.borrow().is_empty()
    }

    fn next_id(&self) -> i32 {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        id
    }
}

impl PlayerDao for InMemoryPlayerDao {
    fn create_player(&self, player: &CreatePlayer) -> Result<(), DaoError> {
        let mut details = PlayerDetails::new();
        details.fill_full(
            self.next_id(),
            &player.username,
            &player.mission,
            &player.figure,
            "",
            &player.email,
            1,
            player.credits,
            &player.sex,
            "",
            "",
            &player.birthday,
            0,
            "",
            0,
        );
        details.set_password(&player.password);

        self.by_username
            .borrow_mut()
            .insert(player.username.clone(), details);
        Ok(())
    }

    fn details_by_id(&self, user_id: i32) -> Result<Option<PlayerDetails>, DaoError> {
        Ok(self
            .by_username
            .borrow()
            .values()
            .find(|details| details.id() == user_id)
            .cloned())
    }

    fn login(&self, username: &str, password: &str) -> Result<Option<LoginResult>, DaoError> {
        let Some(details) = self.by_username.borrow().get(username).cloned() else {
            return Ok(None);
        };

        if details.password() != password {
            return Ok(None);
        }

        Ok(Some(LoginResult::new(details, true)))
    }

    fn id_by_username(&self, username: &str) -> Result<Option<i32>, DaoError> {
        Ok(self
            .by_username
            .borrow()
            .get(username)
            .map(PlayerDetails::id))
    }

    fn is_name_taken(&self, name: &str) -> Result<bool, DaoError> {
        Ok(self.by_username.borrow().contains_key(name))
    }

    fn update_player(&self, details: &PlayerDetails) -> Result<(), DaoError> {
        let mut players = self.by_username.borrow_mut();

        if players.contains_key(details.username()) {
            players.insert(details.username().to_owned(), details.clone());
        }

        Ok(())
    }

    fn update_last_login(&self, details: &PlayerDetails) -> Result<(), DaoError> {
        let mut players = self.by_username.borrow_mut();
        let Some(stored) = players.get(details.username()).cloned() else {
            return Ok(());
        };

        let mut updated = stored.clone();
        updated.fill_full(
            stored.id(),
            stored.username(),
            stored.mission(),
            stored.figure(),
            stored.pool_figure(),
            stored.email(),
            stored.rank(),
            stored.credits(),
            stored.sex(),
            stored.country(),
            stored.badge(),
            stored.birthday(),
            self.last_login_time.get(),
            stored.personal_greeting(),
            stored.tickets(),
        );
        updated.set_password(stored.password());
        players.insert(updated.username().to_owned(), updated);
        Ok(())
    }

    fn permissions(&self) -> Result<Vec<Permission>, DaoError> {
        Ok(Vec::new())
    }

    fn details_by_username(&self, username: &str) -> Result<Option<PlayerDetails>, DaoError> {
        Ok(self.by_username.borrow().get(username).cloned())
    }
}

#[cfg(test)]
#[path = "in_memory_player_dao_tests.rs"]
mod tests;
