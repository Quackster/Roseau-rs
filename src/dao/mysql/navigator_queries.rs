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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_private_room_name_search() {
        let query = NavigatorQueries::rooms_by_like_name("lobby");

        assert_eq!(
            query.sql(),
            "SELECT * FROM rooms WHERE name LIKE ? AND room_type = ?"
        );
        assert_eq!(
            query.parameters(),
            &[
                SqlParameter::Text("%lobby%".to_owned()),
                SqlParameter::Integer(0),
            ]
        );
        assert_eq!(NavigatorQueries::room_table(), "rooms");
    }
}
