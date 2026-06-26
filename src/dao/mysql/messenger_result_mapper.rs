use crate::dao::mysql::entity::{MessengerFriendshipRow, MessengerMessageRow, MessengerRequestRow};
use crate::dao::mysql::mapper::messenger_message_from_row;
use crate::dao::mysql::SqlExecutionResult;
use crate::dao::DaoError;
use crate::game::messenger::{MessengerMessage, MessengerUser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessengerResultMapper;

impl MessengerResultMapper {
    pub fn friends(
        result: SqlExecutionResult,
        user_id: i32,
    ) -> Result<Vec<MessengerUser>, DaoError> {
        result.map_rows(|row| {
            let friendship = MessengerFriendshipRow::try_from(row)?;
            let friend_id = if friendship.sender == user_id {
                friendship.receiver
            } else {
                friendship.sender
            };
            Ok(MessengerUser::new(friend_id))
        })
    }

    pub fn requests(result: SqlExecutionResult) -> Result<Vec<MessengerUser>, DaoError> {
        result.map_rows(|row| {
            let request = MessengerRequestRow::try_from(row)?;
            Ok(MessengerUser::new(request.from_id))
        })
    }

    pub fn request_exists(result: SqlExecutionResult) -> Result<bool, DaoError> {
        result.has_rows()
    }

    pub fn unread_messages(result: SqlExecutionResult) -> Result<Vec<MessengerMessage>, DaoError> {
        result.map_rows(|row| {
            let message_row = MessengerMessageRow::try_from(row)?;
            Ok(messenger_message_from_row(&message_row))
        })
    }

    pub fn created_message_id(result: SqlExecutionResult) -> Result<i32, DaoError> {
        result.require_i32_insert_id("message id")
    }
}

#[cfg(test)]
#[path = "messenger_result_mapper_tests.rs"]
mod tests;
