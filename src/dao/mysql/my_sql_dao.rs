use crate::config::Config;
use crate::dao::DaoError;
use crate::game::item::ItemDefinition;
use crate::game::room::model::RoomModel;
use crate::game::GameVariables;
use std::collections::HashMap;

use super::{
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::DatabaseEngine;
    use std::cell::RefCell;

    #[test]
    fn creates_dao_from_config_and_tracks_connection_state() {
        let config = Config::parse(
            r#"
            [Database]
            type=sqlite
            path=roseau.sqlite
            "#,
        )
        .unwrap();

        let mut dao = MySqlDao::from_config(&config).unwrap();

        assert!(!dao.is_connected());
        assert_eq!(dao.storage().connection_url(), "sqlite:roseau.sqlite");
        assert!(dao.connect().unwrap());
        assert!(dao.is_connected());
    }

    #[test]
    fn plans_java_connection_logs_and_facade_construction_order() {
        let storage = Storage::new(
            DatabaseEngine::MySql,
            "db",
            3306,
            "roseau",
            "secret",
            "hotel",
            "",
            "",
        );
        let dao = MySqlDao::new(storage);

        assert_eq!(
            dao.connect_effects(true),
            vec![
                MySqlDaoEffect::LogLine("Connecting to mysql database".to_owned()),
                MySqlDaoEffect::ConnectStorage,
                MySqlDaoEffect::LogLine("Connection to mysql was a success".to_owned()),
                MySqlDaoEffect::LogLine(String::new()),
            ]
        );
        assert_eq!(
            dao.connect_effects(false),
            vec![
                MySqlDaoEffect::LogLine("Connecting to mysql database".to_owned()),
                MySqlDaoEffect::ConnectStorage,
                MySqlDaoEffect::LogLine("Could not connect".to_owned()),
                MySqlDaoEffect::LogLine(String::new()),
            ]
        );
        assert_eq!(
            dao.construction_effects(),
            vec![
                MySqlDaoEffect::LogLine("Connecting to mysql database".to_owned()),
                MySqlDaoEffect::ConnectStorage,
                MySqlDaoEffect::LogLine("Could not connect".to_owned()),
                MySqlDaoEffect::LogLine(String::new()),
                MySqlDaoEffect::ConstructPlayerDao,
                MySqlDaoEffect::ConstructRoomDao,
                MySqlDaoEffect::ConstructItemDao,
                MySqlDaoEffect::ConstructCatalogueDao,
                MySqlDaoEffect::ConstructInventoryDao,
                MySqlDaoEffect::ConstructNavigatorDao,
                MySqlDaoEffect::ConstructMessengerDao,
            ]
        );
    }

    #[derive(Debug, Default)]
    struct RecordingConnector {
        urls: RefCell<Vec<String>>,
        result: RefCell<Option<Result<(), DaoError>>>,
    }

    impl RecordingConnector {
        fn succeed() -> Self {
            Self {
                urls: RefCell::new(Vec::new()),
                result: RefCell::new(Some(Ok(()))),
            }
        }

        fn fail(message: &str) -> Self {
            Self {
                urls: RefCell::new(Vec::new()),
                result: RefCell::new(Some(Err(DaoError::new(message)))),
            }
        }
    }

    impl StorageConnector for RecordingConnector {
        fn connect(&self, storage: &Storage) -> Result<(), DaoError> {
            self.urls
                .borrow_mut()
                .push(storage.connection_url().to_owned());
            self.result
                .borrow_mut()
                .take()
                .unwrap_or_else(|| Err(DaoError::new("missing connection result")))
        }
    }

    #[test]
    fn updates_connection_state_from_storage_connector_result() {
        let storage = Storage::new(DatabaseEngine::MySql, "db", 3306, "", "", "hotel", "", "");
        let mut dao = MySqlDao::new(storage);
        let connector = RecordingConnector::succeed();

        let outcome = dao.connect_with(&connector);

        assert_eq!(outcome, StorageConnectionOutcome::Connected);
        assert!(dao.is_connected());
        assert_eq!(
            connector.urls.into_inner(),
            vec!["mysql://db:3306/hotel".to_owned()]
        );
    }

    #[test]
    fn records_connector_failure_without_marking_dao_connected() {
        let storage = Storage::new(DatabaseEngine::MySql, "db", 3306, "", "", "hotel", "", "");
        let mut dao = MySqlDao::new(storage);
        let connector = RecordingConnector::fail("database unavailable");

        let outcome = dao.connect_with(&connector);

        assert_eq!(
            outcome,
            StorageConnectionOutcome::Failed {
                message: "database unavailable".to_owned(),
            }
        );
        assert!(!dao.is_connected());
    }

    #[test]
    fn reports_connection_outcome_with_java_connection_effects() {
        let storage = Storage::new(DatabaseEngine::MySql, "db", 3306, "", "", "hotel", "", "");
        let mut dao = MySqlDao::new(storage);
        let connector = RecordingConnector::succeed();

        let report = dao.connect_report_with(&connector);

        assert!(report.connected());
        assert_eq!(report.error(), None);
        assert_eq!(
            report.effects(),
            &[
                MySqlDaoEffect::LogLine("Connecting to mysql database".to_owned()),
                MySqlDaoEffect::ConnectStorage,
                MySqlDaoEffect::LogLine("Connection to mysql was a success".to_owned()),
                MySqlDaoEffect::LogLine(String::new()),
            ]
        );
    }

    #[test]
    fn constructs_typed_facade_bundle_from_runtime_context() {
        use crate::dao::mysql::{SqlExecutionPlan, SqlExecutionResult};
        use crate::game::item::ItemDefinition;
        use crate::game::room::model::RoomModel;

        #[derive(Debug, Clone, Copy)]
        struct NoopExecutor;

        impl SqlExecutor for NoopExecutor {
            fn execute(&self, _plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
                Err(DaoError::new("not executed"))
            }
        }

        let config = Config::parse(
            r#"
            [Register]
            user.name.chars=abc123
            user.default.credits=100
            messenger.greeting=Welcome

            [Scheduler]
            credits.every.x.secs=600
            credits.every.x.amount=10

            [Bot]
            bot.response.delay=1500

            [Player]
            carry.drink.time=180
            carry.drink.interval=12
            talking.lookat.distance=30
            talking.lookat.reset=6
            afk.room.kick=1800

            [Debug]
            debug.enable=false
            "#,
        )
        .unwrap();
        let variables = GameVariables::from_config(&config).unwrap();
        let storage = Storage::new(DatabaseEngine::MySql, "db", 3306, "", "", "hotel", "", "");
        let dao = MySqlDao::new(storage);
        let definition = ItemDefinition::new(5, "chair", "red", 1, 1, 1.0, "SFC", "Chair", "", "");
        let model = RoomModel::new("model_a", "00\r\n00", 0, 0, 0, 2, false, false).unwrap();

        let facades = dao.construct_facades(
            NoopExecutor,
            &variables,
            [(5, definition)].into_iter().collect(),
            [(model.name().to_owned(), model)].into_iter().collect(),
            "alice",
            1234,
        );

        assert_eq!(facades.player().default_credits(), 100);
        assert!(facades.item().definitions_cache().contains_key(&5));
        assert!(facades.room().models().contains_key("model_a"));
        assert_eq!(facades.navigator().owner_name(), "alice");
    }
}
