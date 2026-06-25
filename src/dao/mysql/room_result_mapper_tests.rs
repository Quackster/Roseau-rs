use crate::dao::mysql::{RoomResultMapper, SqlExecutionResult, SqlRow, SqlValue};
use crate::game::room::model::Position;
use crate::game::room::settings::RoomType;

fn room_row(id: i32, name: &str, order_id: i32, users_now: i32) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("name", SqlValue::Text(name.to_owned())),
        ("order_id", SqlValue::Integer(order_id)),
        (
            "room_type",
            SqlValue::Integer(RoomType::Private.type_code()),
        ),
        ("enabled", SqlValue::Integer(1)),
        ("hidden", SqlValue::Integer(0)),
        ("owner_id", SqlValue::Integer(5)),
        ("description", SqlValue::Text("Private room".to_owned())),
        ("password", SqlValue::Text(String::new())),
        ("state", SqlValue::Integer(0)),
        ("show_owner_name", SqlValue::Integer(1)),
        ("allsuperuser", SqlValue::Integer(0)),
        ("users_now", SqlValue::Integer(users_now)),
        ("users_max", SqlValue::Integer(25)),
        ("cct", SqlValue::Text("hh_room".to_owned())),
        ("model", SqlValue::Text("model_a".to_owned())),
        ("wallpaper", SqlValue::Text("101".to_owned())),
        ("floor", SqlValue::Text("201".to_owned())),
    ])
}

fn connection_row() -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("room_id", SqlValue::Integer(10)),
        ("to_id", SqlValue::Integer(20)),
        ("coordinates", SqlValue::Text("2,3".to_owned())),
        ("door_x", SqlValue::Integer(4)),
        ("door_y", SqlValue::Integer(5)),
        ("door_z", SqlValue::Integer(1)),
        ("door_rotation", SqlValue::Integer(2)),
    ])
}

fn model_row() -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Text("model_a".to_owned())),
        ("door_x", SqlValue::Integer(1)),
        ("door_y", SqlValue::Integer(2)),
        ("door_z", SqlValue::Integer(0)),
        ("door_dir", SqlValue::Integer(4)),
        ("heightmap", SqlValue::Text("00\r\n00".to_owned())),
        ("has_pool", SqlValue::Integer(1)),
        ("disable_height_check", SqlValue::Integer(0)),
    ])
}

fn bot_row(walk_to: &str, responses: &str, triggers: &str) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(3)),
        ("room_id", SqlValue::Integer(10)),
        ("name", SqlValue::Text("Guide".to_owned())),
        ("figure", SqlValue::Text("hr-100".to_owned())),
        ("motto", SqlValue::Text("Welcome".to_owned())),
        ("start_x", SqlValue::Integer(1)),
        ("start_y", SqlValue::Integer(2)),
        ("start_z", SqlValue::Integer(0)),
        ("start_rotation", SqlValue::Integer(4)),
        ("walk_to", SqlValue::Text(walk_to.to_owned())),
        ("messages", SqlValue::Text(String::new())),
        ("triggers", SqlValue::Text(triggers.to_owned())),
        ("responses", SqlValue::Text(responses.to_owned())),
    ])
}

#[test]
fn maps_room_rows_to_data_and_summaries() {
    let result = SqlExecutionResult::rows([room_row(10, "Cafe", 2, 3)]);

    let rooms = RoomResultMapper::rooms(result, "alice").unwrap();
    let summaries = RoomResultMapper::room_summaries(
        SqlExecutionResult::rows([room_row(11, "Club", 4, -1)]),
        "bob",
    )
    .unwrap();

    assert_eq!(rooms[0].id(), 10);
    assert_eq!(rooms[0].owner_name(), "alice");
    assert_eq!(summaries[0].data().name(), "Club");
    assert_eq!(summaries[0].order_id(), 4);
    assert_eq!(summaries[0].player_count(), 0);
}

#[test]
fn maps_optional_room_and_empty_rows() {
    let room = RoomResultMapper::optional_room(
        SqlExecutionResult::rows([room_row(10, "Cafe", 2, 3)]),
        "alice",
    )
    .unwrap()
    .unwrap();

    assert_eq!(room.name(), "Cafe");
    assert!(
        RoomResultMapper::optional_room(SqlExecutionResult::rows([]), "alice")
            .unwrap()
            .is_none()
    );
}

#[test]
fn maps_ids_rights_connections_models_and_created_id() {
    assert_eq!(
        RoomResultMapper::public_room_ids(SqlExecutionResult::rows([SqlRow::new([(
            "id",
            SqlValue::Integer(42),
        )])]))
        .unwrap(),
        vec![42]
    );
    assert_eq!(
        RoomResultMapper::room_rights(SqlExecutionResult::rows([SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("user_id", SqlValue::Integer(7)),
            ("room_id", SqlValue::Integer(10)),
        ])]))
        .unwrap(),
        vec![7]
    );

    let connections =
        RoomResultMapper::room_connections(SqlExecutionResult::rows([connection_row()])).unwrap();
    assert_eq!(connections[0].to_id(), 20);
    assert_eq!(
        connections[0].door_position(),
        Position::with_rotation(4, 5, 1.0, 2)
    );

    let models = RoomResultMapper::room_models(SqlExecutionResult::rows([model_row()])).unwrap();
    assert!(models["model_a"].has_pool());
    assert_eq!(
        RoomResultMapper::created_room_id(SqlExecutionResult::insert_id(55)).unwrap(),
        55
    );
}

#[test]
fn maps_bot_rows_with_java_split_rules() {
    let bots = RoomResultMapper::bots(SqlExecutionResult::rows([bot_row(
        "2,3 4,5",
        "Hello|Bye",
        "hi,bye",
    )]))
    .unwrap();

    assert_eq!(bots.len(), 1);
    assert_eq!(bots[0].details().username(), "Guide");
    assert_eq!(bots[0].details().sex(), "Male");
    assert_eq!(
        bots[0].start_position(),
        Position::with_rotation(1, 2, 0.0, 4)
    );
    assert_eq!(bots[0].positions(), &[(2, 3), (4, 5)]);
    assert_eq!(bots[0].responses(), &["Hello".to_owned(), "Bye".to_owned()]);
    assert_eq!(bots[0].triggers(), &["hi".to_owned(), "bye".to_owned()]);
}

#[test]
fn keeps_single_response_or_trigger_as_one_entry() {
    let bots =
        RoomResultMapper::bots(SqlExecutionResult::rows([bot_row("", "Hello", "hi")])).unwrap();

    assert!(bots[0].positions().is_empty());
    assert_eq!(bots[0].responses(), &["Hello".to_owned()]);
    assert_eq!(bots[0].triggers(), &["hi".to_owned()]);
}

#[test]
fn rejects_wrong_result_kind_invalid_columns_and_large_insert_id() {
    assert_eq!(
        RoomResultMapper::rooms(SqlExecutionResult::affected_rows(1), "alice")
            .unwrap_err()
            .message(),
        "SQL execution result contains affected rows, expected read rows"
    );
    assert_eq!(
        RoomResultMapper::public_room_ids(SqlExecutionResult::rows([SqlRow::new([(
            "id",
            SqlValue::Text("bad".to_owned()),
        )])]))
        .unwrap_err()
        .message(),
        "Missing or invalid SQL column `id` as i32"
    );
    assert_eq!(
        RoomResultMapper::bots(SqlExecutionResult::rows([bot_row("bad", "Hello", "hi")]))
            .unwrap_err()
            .message(),
        "Invalid bot walk target: position is missing comma delimiter"
    );
    assert_eq!(
        RoomResultMapper::created_room_id(SqlExecutionResult::insert_id(i64::from(i32::MAX) + 1))
            .unwrap_err()
            .message(),
        "Generated room id 2147483648 exceeds i32"
    );
}
