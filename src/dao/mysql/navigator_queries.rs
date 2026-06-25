use crate::dao::mysql::entity::RoomRow;
use crate::dao::mysql::{SqlParameter, SqlQuery};
use crate::game::room::settings::RoomType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigatorQueries;

impl NavigatorQueries {
    pub fn rooms_by_like_name(name: &str) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM rooms WHERE name LIKE ? AND room_type = ?",
            [
                SqlParameter::Text(format!("%{name}%")),
                SqlParameter::Integer(RoomType::Private.type_code()),
            ],
        )
    }

    pub fn room_table() -> &'static str {
        RoomRow::TABLE
    }
}
