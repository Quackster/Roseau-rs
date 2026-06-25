use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomRow {
    pub id: i32,
    pub name: String,
    pub order_id: i32,
    pub room_type: i32,
    pub enabled: bool,
    pub hidden: bool,
    pub owner_id: i32,
    pub description: String,
    pub password: String,
    pub state: i32,
    pub show_owner_name: bool,
    pub all_super_user: bool,
    pub users_now: i32,
    pub users_max: i32,
    pub cct: String,
    pub model: String,
    pub wallpaper: String,
    pub floor: String,
}

impl RoomRow {
    pub const TABLE: &'static str = "rooms";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        name: impl Into<String>,
        order_id: i32,
        room_type: i32,
        enabled: bool,
        hidden: bool,
        owner_id: i32,
        description: impl Into<String>,
        password: impl Into<String>,
        state: i32,
        show_owner_name: bool,
        all_super_user: bool,
        users_now: i32,
        users_max: i32,
        cct: impl Into<String>,
        model: impl Into<String>,
        wallpaper: impl Into<String>,
        floor: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            order_id,
            room_type,
            enabled,
            hidden,
            owner_id,
            description: description.into(),
            password: password.into(),
            state,
            show_owner_name,
            all_super_user,
            users_now,
            users_max,
            cct: cct.into(),
            model: model.into(),
            wallpaper: wallpaper.into(),
            floor: floor.into(),
        }
    }
}

impl TryFrom<&SqlRow> for RoomRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_string("name")?,
            row.required_i32("order_id")?,
            row.required_i32("room_type")?,
            row.required_bool("enabled")?,
            row.required_bool("hidden")?,
            row.required_i32("owner_id")?,
            row.required_string("description")?,
            row.required_string("password")?,
            row.required_i32("state")?,
            row.required_bool("show_owner_name")?,
            row.required_bool("allsuperuser")?,
            row.required_i32("users_now")?,
            row.required_i32("users_max")?,
            row.required_string("cct")?,
            row.required_string("model")?,
            row.required_string("wallpaper")?,
            row.required_string("floor")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_room_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("name", SqlValue::Text("Lobby".to_owned())),
            ("order_id", SqlValue::Integer(2)),
            ("room_type", SqlValue::Integer(1)),
            ("enabled", SqlValue::Integer(1)),
            ("hidden", SqlValue::Integer(0)),
            ("owner_id", SqlValue::Integer(5)),
            ("description", SqlValue::Text("Public room".to_owned())),
            ("password", SqlValue::Text("".to_owned())),
            ("state", SqlValue::Integer(0)),
            ("show_owner_name", SqlValue::Integer(1)),
            ("allsuperuser", SqlValue::Integer(0)),
            ("users_now", SqlValue::Integer(3)),
            ("users_max", SqlValue::Integer(25)),
            ("cct", SqlValue::Text("hh_room".to_owned())),
            ("model", SqlValue::Text("model_a".to_owned())),
            ("wallpaper", SqlValue::Text("101".to_owned())),
            ("floor", SqlValue::Text("201".to_owned())),
        ]);

        assert_eq!(
            RoomRow::try_from(&row).unwrap(),
            RoomRow::new(
                1,
                "Lobby",
                2,
                1,
                true,
                false,
                5,
                "Public room",
                "",
                0,
                true,
                false,
                3,
                25,
                "hh_room",
                "model_a",
                "101",
                "201",
            )
        );
    }

    #[test]
    fn reports_invalid_room_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("name", SqlValue::Text("Lobby".to_owned())),
        ]);

        assert_eq!(
            RoomRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `order_id` as i32"
        );
    }
}
