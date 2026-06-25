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
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlRow, SqlValue};

    fn friendship_row(sender: i32, receiver: i32) -> SqlRow {
        SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("sender", SqlValue::Integer(sender)),
            ("receiver", SqlValue::Integer(receiver)),
        ])
    }

    fn request_row(from_id: i32, to_id: i32) -> SqlRow {
        SqlRow::new([
            ("id", SqlValue::Integer(4)),
            ("to_id", SqlValue::Integer(to_id)),
            ("from_id", SqlValue::Integer(from_id)),
        ])
    }

    fn message_row(id: i32, from_id: i32, to_id: i32) -> SqlRow {
        SqlRow::new([
            ("id", SqlValue::Integer(id)),
            ("from_id", SqlValue::Integer(from_id)),
            ("to_id", SqlValue::Integer(to_id)),
            ("time_sent", SqlValue::Long(1234)),
            ("message", SqlValue::Text("hello".to_owned())),
            ("unread", SqlValue::Integer(1)),
        ])
    }

    #[test]
    fn maps_friendships_to_other_user_ids() {
        let result = SqlExecutionResult::rows([friendship_row(7, 10), friendship_row(11, 7)]);

        let friends = MessengerResultMapper::friends(result, 7).unwrap();

        assert_eq!(friends.len(), 2);
        assert_eq!(friends[0].user_id(), 10);
        assert_eq!(friends[1].user_id(), 11);
    }

    #[test]
    fn maps_requests_to_requesting_users() {
        let result = SqlExecutionResult::rows([request_row(20, 7), request_row(21, 7)]);

        let requests = MessengerResultMapper::requests(result).unwrap();

        assert_eq!(
            requests
                .iter()
                .map(MessengerUser::user_id)
                .collect::<Vec<_>>(),
            vec![20, 21]
        );
    }

    #[test]
    fn maps_request_existence_from_row_presence() {
        assert!(
            MessengerResultMapper::request_exists(SqlExecutionResult::rows([request_row(1, 2)]))
                .unwrap()
        );
        assert!(!MessengerResultMapper::request_exists(SqlExecutionResult::rows([])).unwrap());
    }

    #[test]
    fn maps_unread_message_rows_to_domain_messages() {
        let result = SqlExecutionResult::rows([message_row(5, 1, 2), message_row(6, 3, 2)]);

        let messages = MessengerResultMapper::unread_messages(result).unwrap();

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].id(), 5);
        assert_eq!(messages[0].from_id(), 1);
        assert_eq!(messages[1].to_id(), 2);
    }

    #[test]
    fn maps_created_message_insert_id() {
        assert_eq!(
            MessengerResultMapper::created_message_id(SqlExecutionResult::insert_id(42)).unwrap(),
            42
        );
    }

    #[test]
    fn rejects_wrong_result_kind_and_large_insert_id() {
        assert_eq!(
            MessengerResultMapper::friends(SqlExecutionResult::affected_rows(1), 7)
                .unwrap_err()
                .message(),
            "SQL execution result contains affected rows, expected read rows"
        );
        assert_eq!(
            MessengerResultMapper::created_message_id(SqlExecutionResult::insert_id(
                i64::from(i32::MAX) + 1
            ))
            .unwrap_err()
            .message(),
            "Generated message id 2147483648 exceeds i32"
        );
    }
}
