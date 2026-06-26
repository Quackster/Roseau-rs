use crate::dao::mysql::entity::{UserPermissionRow, UserRow};
use crate::dao::mysql::mapper::{permission_from_row, player_details_from_row};
use crate::dao::mysql::SqlExecutionResult;
use crate::dao::DaoError;
use crate::game::player::{Permission, PlayerDetails};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerResultMapper;

impl PlayerResultMapper {
    pub fn optional_details(result: SqlExecutionResult) -> Result<Option<PlayerDetails>, DaoError> {
        result
            .optional_first_row()?
            .as_ref()
            .map(|row| {
                let user_row = UserRow::try_from(row)?;
                Ok(player_details_from_row(&user_row))
            })
            .transpose()
    }

    pub fn optional_id(result: SqlExecutionResult) -> Result<Option<i32>, DaoError> {
        result
            .optional_first_row()?
            .map(|row| row.required_i32("id"))
            .transpose()
    }

    pub fn name_taken(result: SqlExecutionResult) -> Result<bool, DaoError> {
        result.has_rows()
    }

    pub fn permissions(result: SqlExecutionResult) -> Result<Vec<Permission>, DaoError> {
        result.map_rows(|row| {
            let permission_row = UserPermissionRow::try_from(row)?;
            Ok(permission_from_row(&permission_row))
        })
    }

    pub fn created_player_id(result: SqlExecutionResult) -> Result<i32, DaoError> {
        result.require_i32_insert_id("player id")
    }
}

#[cfg(test)]
#[path = "player_result_mapper_tests.rs"]
mod tests;
