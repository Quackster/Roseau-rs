use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

use crate::dao::mysql::{
    MySqlRoomDao, SqlExecutionKind, SqlExecutionPlan, SqlExecutionResult, SqlExecutor,
    SqlParameter, SqlRow, SqlValue,
};
use crate::dao::{CreateRoom, DaoError, RoomChatlog, RoomDao};
use crate::game::player::PlayerDetails;
use crate::game::room::model::{Position, RoomModel};
use crate::game::room::settings::RoomType;
use crate::game::room::RoomData;

#[derive(Debug, Default)]
struct RecordingExecutor {
    plans: RefCell<Vec<SqlExecutionPlan>>,
    results: RefCell<VecDeque<Result<SqlExecutionResult, DaoError>>>,
}

impl RecordingExecutor {
    fn push_result(&self, result: SqlExecutionResult) {
        self.results.borrow_mut().push_back(Ok(result));
    }

    fn plans(&self) -> Vec<SqlExecutionPlan> {
        self.plans.borrow().clone()
    }
}

impl SqlExecutor for RecordingExecutor {
    fn execute(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        self.plans.borrow_mut().push(plan);
        self.results
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| Err(DaoError::new("missing queued SQL result")))
    }
}

fn room_row(id: i32, name: &str, room_type: RoomType) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("name", SqlValue::Text(name.to_owned())),
        ("order_id", SqlValue::Integer(id)),
        ("room_type", SqlValue::Integer(room_type.type_code())),
        ("enabled", SqlValue::Integer(1)),
        ("hidden", SqlValue::Integer(0)),
        ("owner_id", SqlValue::Integer(7)),
        ("description", SqlValue::Text("desc".to_owned())),
        ("password", SqlValue::Text(String::new())),
        ("state", SqlValue::Integer(0)),
        ("show_owner_name", SqlValue::Integer(1)),
        ("allsuperuser", SqlValue::Integer(0)),
        ("users_now", SqlValue::Integer(3)),
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

fn bot_row() -> SqlRow {
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
        ("walk_to", SqlValue::Text("2,3".to_owned())),
        ("messages", SqlValue::Text(String::new())),
        ("triggers", SqlValue::Text("hi".to_owned())),
        ("responses", SqlValue::Text("hello".to_owned())),
    ])
}

fn owner() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "hello", "hd-100");
    details
}

fn room_data() -> RoomData {
    RoomData::new(
        42,
        false,
        RoomType::Private,
        7,
        "alice",
        "Room",
        0,
        "",
        25,
        "desc",
        "model_a",
        "default",
        "",
        "",
        false,
        true,
    )
}

fn dao(executor: RecordingExecutor) -> MySqlRoomDao<RecordingExecutor> {
    let model = RoomModel::new("model_a", "00\r\n00", 0, 0, 0, 2, false, false).unwrap();
    MySqlRoomDao::new(
        executor,
        "alice",
        HashMap::from([(model.name().to_owned(), model)]),
        1234,
    )
}

#[test]
fn reads_public_player_single_latest_and_public_ids() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([room_row(
        1,
        "Lobby",
        RoomType::Public,
    )]));
    executor.push_result(SqlExecutionResult::rows([room_row(
        2,
        "Owned",
        RoomType::Private,
    )]));
    executor.push_result(SqlExecutionResult::rows([room_row(
        3,
        "Single",
        RoomType::Private,
    )]));
    executor.push_result(SqlExecutionResult::rows([
        room_row(4, "Keep", RoomType::Private),
        room_row(5, "Skip", RoomType::Private),
    ]));
    executor.push_result(SqlExecutionResult::rows([SqlRow::new([(
        "id",
        SqlValue::Integer(1),
    )])]));
    let dao = dao(executor);

    assert_eq!(dao.public_rooms(false).unwrap()[0].name(), "Lobby");
    assert_eq!(
        dao.player_rooms(&owner(), false).unwrap()[0].owner_name(),
        "alice"
    );
    assert_eq!(dao.room(3, false).unwrap().unwrap().name(), "Single");
    assert_eq!(dao.latest_player_rooms(&[5], 0).unwrap()[0].id(), 4);
    assert_eq!(dao.public_room_ids().unwrap(), vec![1]);

    let plans = dao.executor().plans();
    assert_eq!(
        plans[0].sql(),
        "SELECT * FROM rooms WHERE enabled = ? AND room_type = ? ORDER BY order_id ASC"
    );
    assert_eq!(plans[1].sql(), "SELECT * FROM rooms WHERE owner_id = ?");
    assert_eq!(plans[2].sql(), "SELECT * FROM rooms WHERE id = ? LIMIT 1");
    assert_eq!(
        plans[3].sql(),
        "SELECT * FROM rooms WHERE room_type = ? ORDER BY id DESC LIMIT 11 OFFSET ?"
    );
}

#[test]
fn creates_updates_deletes_and_logs_chat() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::insert_id(77));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = dao(executor);
    let create = CreateRoom::new(&owner(), "Room", "Desc", "model_a", 1, true);

    let created = dao.create_room(&create).unwrap();
    dao.update_room(&room_data()).unwrap();
    dao.delete_room(&room_data()).unwrap();
    dao.save_chatlog(&RoomChatlog::new("alice", 42, "SHOUT", "hello"))
        .unwrap();

    assert_eq!(created.id(), 77);
    assert_eq!(created.owner_name(), "alice");
    let plans = dao.executor().plans();
    assert_eq!(plans[0].kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(
        plans[0].sql(),
        "INSERT INTO rooms (name, description, owner_id, model, state, show_owner_name, room_type) VALUES (?, ?, ?, ?, ?, ?, ?)"
    );
    assert_eq!(
        plans[1].sql(),
        "UPDATE rooms SET name = ?, description = ?, state = ?, password = ?, wallpaper = ?, floor = ?, allsuperuser = ?, show_owner_name = ? WHERE id = ?"
    );
    assert_eq!(plans[2].sql(), "DELETE FROM rooms WHERE id = ?");
    assert_eq!(
        plans[3].sql(),
        "INSERT INTO room_chatlogs (user, room_id, timestamp, message_type, message) VALUES (?, ?, ?, ?, ?)"
    );
    assert_eq!(plans[3].parameters()[2], SqlParameter::Long(1234));
    assert_eq!(plans[3].parameters()[3], SqlParameter::Integer(1));
}

#[test]
fn reads_rights_connections_bots_and_models() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("user_id", SqlValue::Integer(7)),
        ("room_id", SqlValue::Integer(10)),
    ])]));
    executor.push_result(SqlExecutionResult::rows([connection_row()]));
    executor.push_result(SqlExecutionResult::rows([bot_row()]));
    let dao = dao(executor);

    assert_eq!(dao.room_rights(10).unwrap(), vec![7]);
    assert_eq!(
        dao.room_connections(10).unwrap()[0].door_position(),
        Position::with_rotation(4, 5, 1.0, 2)
    );
    assert_eq!(dao.bots(10).unwrap()[0].details().username(), "Guide");
    assert!(dao.model("model_a").unwrap().is_some());
}

#[test]
fn saves_room_rights_as_delete_then_deduped_inserts() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(2));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = dao(executor);

    dao.save_room_rights(42, &[8, 7, 7]).unwrap();

    let plans = dao.executor().plans();
    assert_eq!(plans[0].sql(), "DELETE FROM room_rights WHERE room_id = ?");
    assert_eq!(
        plans[1].parameters(),
        &[SqlParameter::Integer(42), SqlParameter::Integer(7)]
    );
    assert_eq!(
        plans[2].parameters(),
        &[SqlParameter::Integer(42), SqlParameter::Integer(8)]
    );
}

#[test]
fn validates_executor_result_kind_before_mapping() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = dao(executor);

    assert_eq!(
        dao.public_rooms(false).unwrap_err().message(),
        "SQL execution kind ReadRows returned affected rows result"
    );
}
