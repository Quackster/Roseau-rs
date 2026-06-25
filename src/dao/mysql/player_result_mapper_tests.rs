use super::*;
use crate::dao::mysql::{SqlRow, SqlValue};

fn user_row(id: i32, username: &str) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("username", SqlValue::Text(username.to_owned())),
        ("password", SqlValue::Text("hash".to_owned())),
        ("rank", SqlValue::Integer(4)),
        ("mission", SqlValue::Text("hello".to_owned())),
        ("figure", SqlValue::Text("hd-100".to_owned())),
        ("pool_figure", SqlValue::Text("pool".to_owned())),
        ("email", SqlValue::Text("alice@example.test".to_owned())),
        ("credits", SqlValue::Integer(55)),
        ("sex", SqlValue::Text("F".to_owned())),
        ("country", SqlValue::Text("UK".to_owned())),
        ("badge", SqlValue::Text("ADM".to_owned())),
        ("birthday", SqlValue::Text("1990-01-01".to_owned())),
        ("join_date", SqlValue::Long(1000)),
        ("last_online", SqlValue::Long(2000)),
        ("personal_greeting", SqlValue::Text("welcome".to_owned())),
        ("tickets", SqlValue::Integer(8)),
    ])
}

fn permission_row(rank: i32, permission: &str, inheritable: i32) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(3)),
        ("rank", SqlValue::Integer(rank)),
        ("permission", SqlValue::Text(permission.to_owned())),
        ("inheritable", SqlValue::Integer(inheritable)),
    ])
}

#[test]
fn maps_first_user_row_to_optional_player_details() {
    let result = SqlExecutionResult::rows([user_row(7, "alice"), user_row(8, "bob")]);

    let details = PlayerResultMapper::optional_details(result)
        .unwrap()
        .unwrap();

    assert_eq!(details.id(), 7);
    assert_eq!(details.username(), "alice");
    assert_eq!(details.password(), "hash");
    assert_eq!(details.personal_greeting(), "welcome");
}

#[test]
fn maps_empty_user_rows_to_none() {
    assert!(
        PlayerResultMapper::optional_details(SqlExecutionResult::rows([]))
            .unwrap()
            .is_none()
    );
    assert!(
        PlayerResultMapper::optional_id(SqlExecutionResult::rows([]))
            .unwrap()
            .is_none()
    );
}

#[test]
fn maps_id_and_name_taken_from_row_presence() {
    assert_eq!(
        PlayerResultMapper::optional_id(SqlExecutionResult::rows([SqlRow::new([(
            "id",
            SqlValue::Integer(42)
        )])]))
        .unwrap(),
        Some(42)
    );
    assert!(
        PlayerResultMapper::name_taken(SqlExecutionResult::rows([SqlRow::new([(
            "id",
            SqlValue::Integer(7)
        )])]))
        .unwrap()
    );
    assert!(!PlayerResultMapper::name_taken(SqlExecutionResult::rows([])).unwrap());
}

#[test]
fn maps_permission_rows_to_domain_permissions() {
    let result = SqlExecutionResult::rows([
        permission_row(4, "room_admin", 1),
        permission_row(6, "answer_call_for_help", 0),
    ]);

    let permissions = PlayerResultMapper::permissions(result).unwrap();

    assert_eq!(permissions.len(), 2);
    assert_eq!(permissions[0].permission(), "room_admin");
    assert!(permissions[0].is_inheritable());
    assert_eq!(permissions[1].rank(), 6);
    assert!(!permissions[1].is_inheritable());
}

#[test]
fn maps_created_player_insert_id() {
    assert_eq!(
        PlayerResultMapper::created_player_id(SqlExecutionResult::insert_id(77)).unwrap(),
        77
    );
}

#[test]
fn rejects_wrong_result_kind_invalid_columns_and_large_insert_id() {
    assert_eq!(
        PlayerResultMapper::optional_details(SqlExecutionResult::affected_rows(1))
            .unwrap_err()
            .message(),
        "SQL execution result contains affected rows, expected read rows"
    );
    assert_eq!(
        PlayerResultMapper::optional_id(SqlExecutionResult::rows([SqlRow::new([(
            "id",
            SqlValue::Text("no".to_owned())
        )])]))
        .unwrap_err()
        .message(),
        "Missing or invalid SQL column `id` as i32"
    );
    assert_eq!(
        PlayerResultMapper::permissions(SqlExecutionResult::rows([SqlRow::new([(
            "id",
            SqlValue::Integer(3)
        )])]))
        .unwrap_err()
        .message(),
        "Missing or invalid SQL column `rank` as i32"
    );
    assert_eq!(
        PlayerResultMapper::created_player_id(SqlExecutionResult::insert_id(
            i64::from(i32::MAX) + 1
        ))
        .unwrap_err()
        .message(),
        "Generated player id 2147483648 exceeds i32"
    );
}
