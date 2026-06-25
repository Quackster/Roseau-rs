use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomPublicConnectionRow {
    pub id: i32,
    pub room_id: i32,
    pub to_id: i32,
    pub coordinates: String,
    pub door_x: i32,
    pub door_y: i32,
    pub door_z: i32,
    pub door_rotation: i32,
}

impl RoomPublicConnectionRow {
    pub const TABLE: &'static str = "room_public_connections";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        room_id: i32,
        to_id: i32,
        coordinates: impl Into<String>,
        door_x: i32,
        door_y: i32,
        door_z: i32,
        door_rotation: i32,
    ) -> Self {
        Self {
            id,
            room_id,
            to_id,
            coordinates: coordinates.into(),
            door_x,
            door_y,
            door_z,
            door_rotation,
        }
    }
}

impl TryFrom<&SqlRow> for RoomPublicConnectionRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_i32("room_id")?,
            row.required_i32("to_id")?,
            row.required_string("coordinates")?,
            row.required_i32("door_x")?,
            row.required_i32("door_y")?,
            row.required_i32("door_z")?,
            row.required_i32("door_rotation")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_room_public_connection_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(10)),
            ("room_id", SqlValue::Integer(20)),
            ("to_id", SqlValue::Integer(30)),
            ("coordinates", SqlValue::Text("5,6".to_owned())),
            ("door_x", SqlValue::Integer(7)),
            ("door_y", SqlValue::Integer(8)),
            ("door_z", SqlValue::Integer(1)),
            ("door_rotation", SqlValue::Integer(2)),
        ]);

        assert_eq!(
            RoomPublicConnectionRow::try_from(&row).unwrap(),
            RoomPublicConnectionRow::new(10, 20, 30, "5,6", 7, 8, 1, 2)
        );
    }

    #[test]
    fn reports_invalid_room_public_connection_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(10)),
            ("room_id", SqlValue::Integer(20)),
        ]);

        assert_eq!(
            RoomPublicConnectionRow::try_from(&row)
                .unwrap_err()
                .message(),
            "Missing or invalid SQL column `to_id` as i32"
        );
    }
}
