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
