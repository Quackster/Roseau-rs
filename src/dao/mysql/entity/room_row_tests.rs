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

#[test]
fn defaults_nullable_room_text_columns_to_empty_strings() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("name", SqlValue::Text("Lobby".to_owned())),
        ("order_id", SqlValue::Integer(2)),
        ("room_type", SqlValue::Integer(1)),
        ("enabled", SqlValue::Integer(1)),
        ("hidden", SqlValue::Integer(0)),
        ("owner_id", SqlValue::Integer(5)),
        ("description", SqlValue::Null),
        ("password", SqlValue::Null),
        ("state", SqlValue::Integer(0)),
        ("show_owner_name", SqlValue::Integer(1)),
        ("allsuperuser", SqlValue::Integer(0)),
        ("users_now", SqlValue::Integer(3)),
        ("users_max", SqlValue::Integer(25)),
        ("cct", SqlValue::Null),
        ("model", SqlValue::Null),
        ("wallpaper", SqlValue::Null),
        ("floor", SqlValue::Null),
    ]);

    let room = RoomRow::try_from(&row).unwrap();

    assert_eq!(room.description, "");
    assert_eq!(room.password, "");
    assert_eq!(room.cct, "");
    assert_eq!(room.model, "");
    assert_eq!(room.wallpaper, "");
    assert_eq!(room.floor, "");
}
