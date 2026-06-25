use super::my_sql_dao_facades::*;
use crate::dao::mysql::{SqlExecutionPlan, SqlExecutionResult};
use crate::dao::DaoError;

#[derive(Debug, Clone, Copy, Default)]
struct NoopExecutor;

impl SqlExecutor for NoopExecutor {
    fn execute(&self, _plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        Err(DaoError::new("not executed"))
    }
}

fn variables() -> GameVariables {
    let config = crate::Config::parse(
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
    GameVariables::from_config(&config).unwrap()
}

fn definition() -> ItemDefinition {
    ItemDefinition::new(5, "chair", "red", 1, 1, 1.0, "SFC", "Chair", "", "")
}

#[test]
fn constructs_all_mysql_facades_from_shared_runtime_context() {
    let model = RoomModel::new("model_a", "00\r\n00", 0, 0, 0, 2, false, false).unwrap();
    let facades = MySqlDaoFacades::new(
        NoopExecutor,
        &variables(),
        [(5, definition())].into_iter().collect(),
        [(model.name().to_owned(), model)].into_iter().collect(),
        "alice",
        1234,
    );

    assert_eq!(facades.player().default_credits(), 100);
    assert_eq!(facades.player().messenger_greeting(), "Welcome");
    assert_eq!(facades.player().now(), 1234);
    assert_eq!(facades.room().owner_name(), "alice");
    assert!(facades.room().models().contains_key("model_a"));
    assert!(facades.item().definitions_cache().contains_key(&5));
    assert!(facades.inventory().definitions().contains_key(&5));
    assert_eq!(facades.navigator().owner_name(), "alice");
    assert_eq!(facades.messenger().now(), 1234);
}
