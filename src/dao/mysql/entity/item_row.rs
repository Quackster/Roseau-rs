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
            row.required_string_or_number("x")?,
            row.required_i32_compatible("y")?,
            row.required_f64_compatible("z")?,
            row.required_i32_compatible("rotation")?,
            row.optional_string("extra_data")?.unwrap_or_default(),
        ))
    }
}

#[cfg(test)]
#[path = "item_row_tests.rs"]
mod tests;
