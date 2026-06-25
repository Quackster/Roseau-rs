use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq)]
pub struct ItemDefinitionRow {
    pub id: i32,
    pub sprite: String,
    pub color: String,
    pub length: i32,
    pub width: i32,
    pub height: f64,
    pub data_class: String,
    pub behaviour: String,
    pub name: String,
    pub description: String,
}

impl ItemDefinitionRow {
    pub const TABLE: &'static str = "item_definitions";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        sprite: impl Into<String>,
        color: impl Into<String>,
        length: i32,
        width: i32,
        height: f64,
        data_class: impl Into<String>,
        behaviour: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            sprite: sprite.into(),
            color: color.into(),
            length,
            width,
            height,
            data_class: data_class.into(),
            behaviour: behaviour.into(),
            name: name.into(),
            description: description.into(),
        }
    }
}

impl TryFrom<&SqlRow> for ItemDefinitionRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_string("sprite")?,
            row.required_string("color")?,
            row.required_i32("length")?,
            row.required_i32("width")?,
            row.required_f64("height")?,
            row.required_string("dataclass")?,
            row.required_string("behaviour")?,
            row.required_string("name")?,
            row.required_string("description")?,
        ))
    }
}

#[cfg(test)]
#[path = "item_definition_row_tests.rs"]
mod tests;
