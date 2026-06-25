use crate::dao::mysql::entity::UserRow;
use crate::dao::mysql::{
    PlayerPasswordQueries, PlayerQueries, PlayerResultMapper, SqlExecutionPlan, SqlExecutionResult,
    SqlExecutor,
};
use crate::dao::{CreatePlayer, DaoError, LoginResult, PlayerDao};
use crate::game::player::{Permission, PlayerDetails};

#[derive(Debug)]
pub struct MySqlPlayerDao<E> {
    executor: E,
    password_queries: PlayerPasswordQueries,
    default_credits: i32,
    messenger_greeting: String,
    now: i64,
}

impl<E> MySqlPlayerDao<E> {
    pub fn new(
        executor: E,
        password_queries: PlayerPasswordQueries,
        default_credits: i32,
        messenger_greeting: impl Into<String>,
        now: i64,
    ) -> Self {
        Self {
            executor,
            password_queries,
            default_credits,
            messenger_greeting: messenger_greeting.into(),
            now,
        }
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }

    pub fn password_queries(&self) -> PlayerPasswordQueries {
        self.password_queries
    }

    pub fn default_credits(&self) -> i32 {
        self.default_credits
    }

    pub fn messenger_greeting(&self) -> &str {
        &self.messenger_greeting
    }

    pub fn now(&self) -> i64 {
        self.now
    }

    pub fn set_now(&mut self, now: i64) {
        self.now = now;
    }
}

impl<E: SqlExecutor> MySqlPlayerDao<E> {
    fn execute_plan(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        let result = self.executor.execute(plan.clone())?;
        plan.validate_result(result)
    }
}

impl<E: SqlExecutor> PlayerDao for MySqlPlayerDao<E> {
    fn create_player(&self, player: &CreatePlayer) -> Result<(), DaoError> {
        let plan = self
            .password_queries
            .create_player_plan(
                player,
                self.default_credits,
                &self.messenger_greeting,
                self.now,
            )
            .map_err(PlayerPasswordQueries::hash_error)?;
        let result = self.execute_plan(plan)?;
        PlayerResultMapper::created_player_id(result)?;
        Ok(())
    }

    fn details_by_id(&self, user_id: i32) -> Result<Option<PlayerDetails>, DaoError> {
        let result = self.execute_plan(PlayerQueries::details_by_id(user_id).read_plan())?;
        PlayerResultMapper::optional_details(result)
    }

    fn login(&self, username: &str, password: &str) -> Result<Option<LoginResult>, DaoError> {
        let result = self.execute_plan(PlayerPasswordQueries::login_lookup_plan(username))?;
        let Some(row) = result.optional_first_row()? else {
            return Ok(None);
        };
        let row = UserRow::try_from(&row)?;

        let Some(details) = self
            .password_queries
            .verified_login_details(&row, password)
            .map_err(PlayerPasswordQueries::login_error)?
        else {
            return Ok(None);
        };

        Ok(Some(LoginResult::new(details, true)))
    }

    fn id_by_username(&self, username: &str) -> Result<Option<i32>, DaoError> {
        let result = self.execute_plan(PlayerQueries::id_by_username(username).read_plan())?;
        PlayerResultMapper::optional_id(result)
    }

    fn is_name_taken(&self, name: &str) -> Result<bool, DaoError> {
        let result = self.execute_plan(PlayerQueries::is_name_taken(name).read_plan())?;
        PlayerResultMapper::name_taken(result)
    }

    fn update_player(&self, details: &PlayerDetails) -> Result<(), DaoError> {
        let plan = self
            .password_queries
            .update_player_plan(details)
            .map_err(PlayerPasswordQueries::hash_error)?;
        self.execute_plan(plan)?.require_mutation()
    }

    fn update_last_login(&self, details: &PlayerDetails) -> Result<(), DaoError> {
        self.execute_plan(PlayerQueries::update_last_login(details.id(), self.now).execute_plan())?
            .require_mutation()
    }

    fn permissions(&self) -> Result<Vec<Permission>, DaoError> {
        let result = self.execute_plan(PlayerQueries::permissions().read_plan())?;
        PlayerResultMapper::permissions(result)
    }

    fn details_by_username(&self, username: &str) -> Result<Option<PlayerDetails>, DaoError> {
        let result = self.execute_plan(PlayerQueries::details_by_username(username).read_plan())?;
        PlayerResultMapper::optional_details(result)
    }
}
