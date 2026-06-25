use crate::config::Config;
use crate::dao::mysql::{
    DatabaseEngine, MySqlDao, MySqlDaoEffect, SqlExecutionPlan, SqlExecutionResult, SqlExecutor,
    Storage, StorageConnectionOutcome, StorageConnector,
};
use crate::dao::DaoError;
use crate::game::item::ItemDefinition;
use crate::game::room::model::RoomModel;
use crate::game::GameVariables;
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

#[derive(Debug, Clone, Copy)]
struct NoopExecutor;

impl SqlExecutor for NoopExecutor {
    fn execute(&self, _plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        Err(DaoError::new("not executed"))
    }
}

#[test]
fn constructs_typed_facade_bundle_from_runtime_context() {
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
