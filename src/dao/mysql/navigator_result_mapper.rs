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
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlRow, SqlValue};
    use crate::game::room::settings::RoomType;

    fn room_row(id: i32, name: &str, users_now: i32) -> SqlRow {
        SqlRow::new([
            ("id", SqlValue::Integer(id)),
            ("name", SqlValue::Text(name.to_owned())),
            ("order_id", SqlValue::Integer(2)),
            (
                "room_type",
                SqlValue::Integer(RoomType::Private.type_code()),
            ),
            ("enabled", SqlValue::Integer(1)),
            ("hidden", SqlValue::Integer(0)),
            ("owner_id", SqlValue::Integer(5)),
            ("description", SqlValue::Text("Private room".to_owned())),
            ("password", SqlValue::Text(String::new())),
            ("state", SqlValue::Integer(0)),
            ("show_owner_name", SqlValue::Integer(1)),
            ("allsuperuser", SqlValue::Integer(0)),
            ("users_now", SqlValue::Integer(users_now)),
            ("users_max", SqlValue::Integer(25)),
            ("cct", SqlValue::Text("hh_room".to_owned())),
            ("model", SqlValue::Text("model_a".to_owned())),
            ("wallpaper", SqlValue::Text("101".to_owned())),
            ("floor", SqlValue::Text("201".to_owned())),
        ])
    }

    #[test]
    fn maps_room_rows_to_room_data_results() {
        let result = SqlExecutionResult::rows([room_row(10, "Cafe", 3)]);

        let rooms = NavigatorResultMapper::rooms_by_like_name(result, "alice").unwrap();

        assert_eq!(rooms.len(), 1);
        assert_eq!(rooms[0].id(), 10);
        assert_eq!(rooms[0].name(), "Cafe");
        assert_eq!(rooms[0].owner_name(), "alice");
    }

    #[test]
    fn maps_room_rows_to_room_summaries_with_counts() {
        let result = SqlExecutionResult::rows([room_row(10, "Cafe", 3)]);

        let rooms = NavigatorResultMapper::room_summaries_by_like_name(result, "alice").unwrap();

        assert_eq!(rooms.len(), 1);
        assert_eq!(rooms[0].data().id(), 10);
        assert_eq!(rooms[0].order_id(), 2);
        assert_eq!(rooms[0].player_count(), 3);
    }

    #[test]
    fn negative_user_counts_are_clamped_to_zero_for_summaries() {
        let result = SqlExecutionResult::rows([room_row(10, "Cafe", -1)]);

        let rooms = NavigatorResultMapper::room_summaries_by_like_name(result, "alice").unwrap();

        assert_eq!(rooms[0].player_count(), 0);
    }

    #[test]
    fn rejects_non_row_results_and_invalid_room_columns() {
        assert_eq!(
            NavigatorResultMapper::rooms_by_like_name(
                SqlExecutionResult::affected_rows(1),
                "alice"
            )
            .unwrap_err()
            .message(),
            "SQL execution result contains affected rows, expected read rows"
        );

        let invalid = SqlExecutionResult::rows([SqlRow::new([
            ("id", SqlValue::Integer(10)),
            ("name", SqlValue::Text("Cafe".to_owned())),
        ])]);
        assert_eq!(
            NavigatorResultMapper::rooms_by_like_name(invalid, "alice")
                .unwrap_err()
                .message(),
            "Missing or invalid SQL column `order_id` as i32"
        );
    }
}
