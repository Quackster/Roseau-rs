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
