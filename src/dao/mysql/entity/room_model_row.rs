use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomModelRow {
    pub id: String,
    pub door_x: i32,
    pub door_y: i32,
    pub door_z: i32,
    pub door_dir: i32,
    pub heightmap: String,
    pub has_pool: bool,
    pub disable_height_check: bool,
}

impl RoomModelRow {
    pub const TABLE: &'static str = "room_models";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        door_x: i32,
        door_y: i32,
        door_z: i32,
        door_dir: i32,
        heightmap: impl Into<String>,
        has_pool: bool,
        disable_height_check: bool,
    ) -> Self {
        Self {
            id: id.into(),
            door_x,
            door_y,
            door_z,
            door_dir,
            heightmap: heightmap.into(),
            has_pool,
            disable_height_check,
        }
    }
}

impl TryFrom<&SqlRow> for RoomModelRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_string("id")?,
            row.required_i32("door_x")?,
            row.required_i32("door_y")?,
            row.required_i32("door_z")?,
            row.required_i32("door_dir")?,
            row.required_string("heightmap")?,
            row.required_bool("has_pool")?,
            row.required_bool("disable_height_check")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_room_model_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Text("model_a".to_owned())),
            ("door_x", SqlValue::Integer(1)),
            ("door_y", SqlValue::Integer(2)),
            ("door_z", SqlValue::Integer(0)),
            ("door_dir", SqlValue::Integer(4)),
            ("heightmap", SqlValue::Text("xxx".to_owned())),
            ("has_pool", SqlValue::Integer(1)),
            ("disable_height_check", SqlValue::Integer(0)),
        ]);

        assert_eq!(
            RoomModelRow::try_from(&row).unwrap(),
            RoomModelRow::new("model_a", 1, 2, 0, 4, "xxx", true, false)
        );
    }

    #[test]
    fn reports_invalid_room_model_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Text("model_a".to_owned())),
            ("door_x", SqlValue::Integer(1)),
        ]);

        assert_eq!(
            RoomModelRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `door_y` as i32"
        );
    }
}
