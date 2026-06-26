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
            row.required_string_or_number("x")?,
            row.required_i32("y")?,
            row.required_f64("z")?,
            row.required_i32("rotation")?,
            row.required_i32("definitionid")?,
            row.optional_string("object")?.unwrap_or_default(),
            row.optional_string("data")?.unwrap_or_default(),
        ))
    }
}

#[cfg(test)]
#[path = "room_public_item_row_tests.rs"]
mod tests;
