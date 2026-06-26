use std::collections::HashMap;

use crate::dao::mysql::{
    MySqlCatalogueDao, MySqlInventoryDao, MySqlItemDao, MySqlMessengerDao, MySqlNavigatorDao,
    MySqlPlayerDao, MySqlRoomDao, PlayerPasswordQueries, SqlExecutor,
};
use crate::game::item::ItemDefinition;
use crate::game::room::model::RoomModel;
use crate::game::GameVariables;

#[derive(Debug)]
pub struct MySqlDaoFacades<E> {
    player: MySqlPlayerDao<E>,
    room: MySqlRoomDao<E>,
    item: MySqlItemDao<E>,
    catalogue: MySqlCatalogueDao<E>,
    inventory: MySqlInventoryDao<E>,
    navigator: MySqlNavigatorDao<E>,
    messenger: MySqlMessengerDao<E>,
}

impl<E: SqlExecutor + Clone> MySqlDaoFacades<E> {
    pub fn new(
        executor: E,
        variables: &GameVariables,
        item_definitions: HashMap<i32, ItemDefinition>,
        room_models: HashMap<String, RoomModel>,
        owner_name: impl Into<String>,
        now: i64,
    ) -> Self {
        let owner_name = owner_name.into();
        Self {
            player: MySqlPlayerDao::new(
                executor.clone(),
                PlayerPasswordQueries::java_compatible(),
                variables.user_default_credits(),
                variables.messenger_greeting(),
                now,
            ),
            room: MySqlRoomDao::new(executor.clone(), owner_name.clone(), room_models, now),
            item: MySqlItemDao::new(executor.clone(), item_definitions.clone()),
            catalogue: MySqlCatalogueDao::new(executor.clone()),
            inventory: MySqlInventoryDao::new(executor.clone(), item_definitions),
            navigator: MySqlNavigatorDao::new(executor.clone(), owner_name),
            messenger: MySqlMessengerDao::new(executor, now),
        }
    }
}

impl<E> MySqlDaoFacades<E> {
    pub fn player(&self) -> &MySqlPlayerDao<E> {
        &self.player
    }

    pub fn room(&self) -> &MySqlRoomDao<E> {
        &self.room
    }

    pub fn item(&self) -> &MySqlItemDao<E> {
        &self.item
    }

    pub fn catalogue(&self) -> &MySqlCatalogueDao<E> {
        &self.catalogue
    }

    pub fn inventory(&self) -> &MySqlInventoryDao<E> {
        &self.inventory
    }

    pub fn navigator(&self) -> &MySqlNavigatorDao<E> {
        &self.navigator
    }

    pub fn messenger(&self) -> &MySqlMessengerDao<E> {
        &self.messenger
    }
}

#[cfg(test)]
#[path = "my_sql_dao_facades_tests.rs"]
mod tests;
