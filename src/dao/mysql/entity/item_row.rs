use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq)]
pub struct ItemRow {
    pub id: i32,
    pub user_id: i32,
    pub item_id: i32,
    pub room_id: i32,
    pub x: String,
    pub y: i32,
    pub z: f64,
    pub rotation: i32,
    pub extra_data: String,
}

impl ItemRow {
    pub const TABLE: &'static str = "items";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        user_id: i32,
        item_id: i32,
        room_id: i32,
        x: impl Into<String>,
        y: i32,
        z: f64,
        rotation: i32,
        extra_data: impl Into<String>,
    ) -> Self {
        Self {
            id,
            user_id,
            item_id,
            room_id,
            x: x.into(),
            y,
            z,
            rotation,
            extra_data: extra_data.into(),
        }
    }
}

impl TryFrom<&SqlRow> for ItemRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_i32("user_id")?,
            row.required_i32("item_id")?,
            row.required_i32("room_id")?,
            row.required_string("x")?,
            row.required_i32("y")?,
            row.required_f64("z")?,
            row.required_i32("rotation")?,
            row.required_string("extra_data")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_item_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(10)),
            ("user_id", SqlValue::Integer(3)),
            ("item_id", SqlValue::Integer(5)),
            ("room_id", SqlValue::Integer(7)),
            ("x", SqlValue::Text("1".to_owned())),
            ("y", SqlValue::Integer(2)),
            ("z", SqlValue::Float(0.5)),
            ("rotation", SqlValue::Integer(4)),
            ("extra_data", SqlValue::Text("ON".to_owned())),
        ]);

        assert_eq!(
            ItemRow::try_from(&row).unwrap(),
            ItemRow::new(10, 3, 5, 7, "1", 2, 0.5, 4, "ON")
        );
    }

    #[test]
    fn reports_invalid_item_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(10)),
            ("user_id", SqlValue::Integer(3)),
        ]);

        assert_eq!(
            ItemRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `item_id` as i32"
        );
    }
}
