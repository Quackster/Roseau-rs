use crate::config::Config;
use crate::dao::DaoError;
use crate::game::item::ItemDefinition;
use crate::game::room::model::RoomModel;
use crate::game::GameVariables;
use std::collections::HashMap;

use crate::dao::mysql::{
    MySqlDaoConnectionReport, MySqlDaoEffect, MySqlDaoFacades, SqlExecutor, Storage,
    StorageConnectionOutcome, StorageConnector,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MySqlDao {
    storage: Storage,
    connected: bool,
}

impl MySqlDao {
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
            connected: false,
        }
    }

    pub fn from_config(config: &Config) -> Result<Self, DaoError> {
        Ok(Self::new(Storage::from_config(config)?))
    }

    pub fn connect(&mut self) -> Result<bool, DaoError> {
        self.connected = !self.storage.connection_url().trim().is_empty();
        Ok(self.connected)
    }

    pub fn connect_with<C: StorageConnector>(&mut self, connector: &C) -> StorageConnectionOutcome {
        let outcome = StorageConnectionOutcome::from_result(connector.connect(&self.storage));
        self.connected = outcome.is_connected();
        outcome
    }

    pub fn connect_report_with<C: StorageConnector>(
        &mut self,
        connector: &C,
    ) -> MySqlDaoConnectionReport {
        let outcome = self.connect_with(connector);
        let effects = self.connect_effects(outcome.is_connected());

        MySqlDaoConnectionReport::new(outcome, effects)
    }

    pub fn construction_effects(&self) -> Vec<MySqlDaoEffect> {
        let mut effects = self.connect_effects(self.connected);
        effects.extend([
            MySqlDaoEffect::ConstructPlayerDao,
            MySqlDaoEffect::ConstructRoomDao,
            MySqlDaoEffect::ConstructItemDao,
            MySqlDaoEffect::ConstructCatalogueDao,
            MySqlDaoEffect::ConstructInventoryDao,
            MySqlDaoEffect::ConstructNavigatorDao,
            MySqlDaoEffect::ConstructMessengerDao,
        ]);
        effects
    }

    pub fn construct_facades<E: SqlExecutor + Clone>(
        &self,
        executor: E,
        variables: &GameVariables,
        item_definitions: HashMap<i32, ItemDefinition>,
        room_models: HashMap<String, RoomModel>,
        owner_name: impl Into<String>,
        now: i64,
    ) -> MySqlDaoFacades<E> {
        MySqlDaoFacades::new(
            executor,
            variables,
            item_definitions,
            room_models,
            owner_name,
            now,
        )
    }

    pub fn connect_effects(&self, connected: bool) -> Vec<MySqlDaoEffect> {
        let prefix = self.storage.engine().config_prefix();
        let mut effects = vec![
            MySqlDaoEffect::LogLine(format!("Connecting to {prefix} database")),
            MySqlDaoEffect::ConnectStorage,
        ];

        effects.push(if connected {
            MySqlDaoEffect::LogLine(format!("Connection to {prefix} was a success"))
        } else {
            MySqlDaoEffect::LogLine("Could not connect".to_owned())
        });
        effects.push(MySqlDaoEffect::LogLine(String::new()));
        effects
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }
}
