use crate::dao::mysql::entity::RoomRow;
use crate::dao::mysql::mapper::room_data_from_row;
use crate::dao::mysql::SqlExecutionResult;
use crate::dao::DaoError;
use crate::game::room::{RoomData, RoomSummary};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigatorResultMapper;

impl NavigatorResultMapper {
    pub fn rooms_by_like_name(
        result: SqlExecutionResult,
        owner_name: &str,
    ) -> Result<Vec<RoomData>, DaoError> {
        result.map_rows(|row| {
            let room_row = RoomRow::try_from(row)?;
            Ok(room_data_from_row(&room_row, owner_name))
        })
    }

    pub fn room_summaries_by_like_name(
        result: SqlExecutionResult,
        owner_name: &str,
    ) -> Result<Vec<RoomSummary>, DaoError> {
        result.map_rows(|row| {
            let room_row = RoomRow::try_from(row)?;
            let mut summary = RoomSummary::new(room_data_from_row(&room_row, owner_name));
            summary.set_order_id(room_row.order_id);
            summary.set_player_count(usize::try_from(room_row.users_now).unwrap_or(0));
            Ok(summary)
        })
    }
}

#[cfg(test)]
#[path = "navigator_result_mapper_tests.rs"]
mod tests;
