use crate::dao::mysql::{
    MessengerQueries, MessengerResultMapper, SqlExecutionPlan, SqlExecutionResult, SqlExecutor,
};
use crate::dao::{DaoError, MessengerDao};
use crate::game::messenger::{MessengerMessage, MessengerUser};

#[derive(Debug)]
pub struct MySqlMessengerDao<E> {
    executor: E,
    now: i64,
}

impl<E> MySqlMessengerDao<E> {
    pub fn new(executor: E, now: i64) -> Self {
        Self { executor, now }
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }

    pub fn now(&self) -> i64 {
        self.now
    }

    pub fn set_now(&mut self, now: i64) {
        self.now = now;
    }
}

impl<E: SqlExecutor> MySqlMessengerDao<E> {
    fn execute_plan(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        let result = self.executor.execute(plan.clone())?;
        plan.validate_result(result)
    }

    fn execute_mutation(&self, plan: SqlExecutionPlan) -> Result<(), DaoError> {
        self.execute_plan(plan)?.require_mutation()
    }
}

impl<E: SqlExecutor> MessengerDao for MySqlMessengerDao<E> {
    fn friends(&self, user_id: i32) -> Result<Vec<MessengerUser>, DaoError> {
        let result = self.execute_plan(MessengerQueries::friends(user_id).read_plan())?;
        MessengerResultMapper::friends(result, user_id)
    }

    fn requests(&self, user_id: i32) -> Result<Vec<MessengerUser>, DaoError> {
        let result = self.execute_plan(MessengerQueries::requests(user_id).read_plan())?;
        MessengerResultMapper::requests(result)
    }

    fn new_request(&self, from_id: i32, to_id: i32) -> Result<bool, DaoError> {
        if self.request_exists(from_id, to_id)? {
            return Ok(false);
        }

        self.execute_mutation(MessengerQueries::create_request(from_id, to_id).execute_plan())?;
        Ok(true)
    }

    fn remove_request(&self, from_id: i32, to_id: i32) -> Result<(), DaoError> {
        self.execute_mutation(MessengerQueries::remove_request(from_id, to_id).execute_plan())
    }

    fn new_friend(&self, sender: i32, receiver: i32) -> Result<bool, DaoError> {
        self.execute_mutation(MessengerQueries::create_friend(sender, receiver).execute_plan())?;
        Ok(true)
    }

    fn remove_friend(&self, friend_id: i32, user_id: i32) -> Result<(), DaoError> {
        self.execute_mutation(MessengerQueries::remove_friend(friend_id, user_id).execute_plan())
    }

    fn request_exists(&self, from_id: i32, to_id: i32) -> Result<bool, DaoError> {
        let result =
            self.execute_plan(MessengerQueries::request_exists(from_id, to_id).read_plan())?;
        MessengerResultMapper::request_exists(result)
    }

    fn new_message(&self, from_id: i32, to_id: i32, message: &str) -> Result<i32, DaoError> {
        let result = self.execute_plan(
            MessengerQueries::create_message(from_id, to_id, self.now, message)
                .insert_returning_id_plan(),
        )?;
        MessengerResultMapper::created_message_id(result)
    }

    fn unread_messages(&self, user_id: i32) -> Result<Vec<MessengerMessage>, DaoError> {
        let result = self.execute_plan(MessengerQueries::unread_messages(user_id).read_plan())?;
        MessengerResultMapper::unread_messages(result)
    }

    fn mark_message_read(&self, message_id: i32) -> Result<(), DaoError> {
        self.execute_mutation(MessengerQueries::mark_message_read(message_id).execute_plan())
    }
}
