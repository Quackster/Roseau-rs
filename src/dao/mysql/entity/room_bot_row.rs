use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomBotRow {
    pub id: i32,
    pub room_id: i32,
    pub name: String,
    pub figure: String,
    pub motto: String,
    pub start_x: i32,
    pub start_y: i32,
    pub start_z: i32,
    pub start_rotation: i32,
    pub walk_to: String,
    pub messages: String,
    pub triggers: String,
    pub responses: String,
}

impl RoomBotRow {
    pub const TABLE: &'static str = "room_bots";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        room_id: i32,
        name: impl Into<String>,
        figure: impl Into<String>,
        motto: impl Into<String>,
        start_x: i32,
        start_y: i32,
        start_z: i32,
        start_rotation: i32,
        walk_to: impl Into<String>,
        messages: impl Into<String>,
        triggers: impl Into<String>,
        responses: impl Into<String>,
    ) -> Self {
        Self {
            id,
            room_id,
            name: name.into(),
            figure: figure.into(),
            motto: motto.into(),
            start_x,
            start_y,
            start_z,
            start_rotation,
            walk_to: walk_to.into(),
            messages: messages.into(),
            triggers: triggers.into(),
            responses: responses.into(),
        }
    }
}

impl TryFrom<&SqlRow> for RoomBotRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_i32("room_id")?,
            row.required_string("name")?,
            row.required_string("figure")?,
            row.required_string("motto")?,
            row.required_i32("start_x")?,
            row.required_i32("start_y")?,
            row.required_i32("start_z")?,
            row.required_i32("start_rotation")?,
            row.required_string("walk_to")?,
            row.required_string("messages")?,
            row.required_string("triggers")?,
            row.required_string("responses")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_room_bot_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("room_id", SqlValue::Integer(2)),
            ("name", SqlValue::Text("Guide".to_owned())),
            ("figure", SqlValue::Text("hr-100".to_owned())),
            ("motto", SqlValue::Text("Welcome".to_owned())),
            ("start_x", SqlValue::Integer(3)),
            ("start_y", SqlValue::Integer(4)),
            ("start_z", SqlValue::Integer(0)),
            ("start_rotation", SqlValue::Integer(2)),
            ("walk_to", SqlValue::Text("5,6".to_owned())),
            ("messages", SqlValue::Text("hi".to_owned())),
            ("triggers", SqlValue::Text("hello".to_owned())),
            ("responses", SqlValue::Text("welcome".to_owned())),
        ]);

        assert_eq!(
            RoomBotRow::try_from(&row).unwrap(),
            RoomBotRow::new(
                1, 2, "Guide", "hr-100", "Welcome", 3, 4, 0, 2, "5,6", "hi", "hello", "welcome",
            )
        );
    }

    #[test]
    fn reports_invalid_room_bot_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("room_id", SqlValue::Integer(2)),
        ]);

        assert_eq!(
            RoomBotRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `name` as String"
        );
    }
}
