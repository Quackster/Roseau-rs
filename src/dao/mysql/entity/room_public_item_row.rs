use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq)]
pub struct RoomPublicItemRow {
    pub id: i32,
    pub model: String,
    pub x: String,
    pub y: i32,
    pub z: f64,
    pub rotation: i32,
    pub definition_id: i32,
    pub object: String,
    pub data: String,
}

impl RoomPublicItemRow {
    pub const TABLE: &'static str = "room_public_items";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        model: impl Into<String>,
        x: impl Into<String>,
        y: i32,
        z: f64,
        rotation: i32,
        definition_id: i32,
        object: impl Into<String>,
        data: impl Into<String>,
    ) -> Self {
        Self {
            id,
            model: model.into(),
            x: x.into(),
            y,
            z,
            rotation,
            definition_id,
            object: object.into(),
            data: data.into(),
        }
    }
}

impl TryFrom<&SqlRow> for RoomPublicItemRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_string("model")?,
            row.required_string("x")?,
            row.required_i32("y")?,
            row.required_f64("z")?,
            row.required_i32("rotation")?,
            row.required_i32("definitionid")?,
            row.required_string("object")?,
            row.required_string("data")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_room_public_item_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(20)),
            ("model", SqlValue::Text("pool_a".to_owned())),
            ("x", SqlValue::Text("4".to_owned())),
            ("y", SqlValue::Integer(5)),
            ("z", SqlValue::Float(1.0)),
            ("rotation", SqlValue::Integer(2)),
            ("definitionid", SqlValue::Integer(8)),
            ("object", SqlValue::Text("chair".to_owned())),
            ("data", SqlValue::Text("ON".to_owned())),
        ]);

        assert_eq!(
            RoomPublicItemRow::try_from(&row).unwrap(),
            RoomPublicItemRow::new(20, "pool_a", "4", 5, 1.0, 2, 8, "chair", "ON")
        );
    }

    #[test]
    fn reports_invalid_room_public_item_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(20)),
            ("model", SqlValue::Text("pool_a".to_owned())),
        ]);

        assert_eq!(
            RoomPublicItemRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `x` as String"
        );
    }
}
