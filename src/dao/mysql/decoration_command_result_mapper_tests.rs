use super::decoration_command_result_mapper::*;
use crate::dao::mysql::{SqlParameter, SqlRow, SqlValue};

fn definitions(behaviour: &str) -> HashMap<i32, ItemDefinition> {
    [(
        5,
        ItemDefinition::new(5, "paper", "red", 1, 1, 1.0, behaviour, "Paper", "", ""),
    )]
    .into_iter()
    .collect()
}

fn item_row(custom_data: &str) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(10)),
        ("user_id", SqlValue::Integer(7)),
        ("item_id", SqlValue::Integer(5)),
        ("room_id", SqlValue::Integer(0)),
        ("x", SqlValue::Text("0".to_owned())),
        ("y", SqlValue::Integer(0)),
        ("z", SqlValue::Float(0.0)),
        ("rotation", SqlValue::Integer(0)),
        ("extra_data", SqlValue::Text(custom_data.to_owned())),
    ])
}

fn effect(decoration: &str) -> IncomingExecutionEffect {
    IncomingExecutionEffect::ApplyDecoration {
        decoration: decoration.to_owned(),
        item_id: 10,
    }
}

#[test]
fn maps_loaded_decoration_item_to_delete_and_room_update_plans() {
    let plans = DecorationCommandResultMapper::plans(
        SqlExecutionResult::rows([item_row("101")]),
        &definitions("V"),
        &effect("wallpaper"),
        42,
    )
    .unwrap();

    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].sql(), "DELETE FROM items WHERE id = ?");
    assert_eq!(plans[0].parameters(), &[SqlParameter::Long(10)]);
    assert_eq!(
        plans[1].sql(),
        "UPDATE rooms SET wallpaper = ? WHERE id = ?"
    );
    assert_eq!(
        plans[1].parameters(),
        &[
            SqlParameter::Text("101".to_owned()),
            SqlParameter::Integer(42)
        ]
    );
}

#[test]
fn ignores_missing_non_decoration_and_unknown_decoration_results() {
    assert_eq!(
        DecorationCommandResultMapper::plans(
            SqlExecutionResult::rows([]),
            &definitions("V"),
            &effect("wallpaper"),
            42,
        )
        .unwrap(),
        Vec::new()
    );
    assert_eq!(
        DecorationCommandResultMapper::plans(
            SqlExecutionResult::rows([item_row("101")]),
            &definitions("SIF"),
            &effect("wallpaper"),
            42,
        )
        .unwrap(),
        Vec::new()
    );
    assert_eq!(
        DecorationCommandResultMapper::plans(
            SqlExecutionResult::rows([item_row("101")]),
            &definitions("V"),
            &effect("ceiling"),
            42,
        )
        .unwrap(),
        Vec::new()
    );
    assert_eq!(
        DecorationCommandResultMapper::plans(
            SqlExecutionResult::rows([item_row("101")]),
            &definitions("V"),
            &IncomingExecutionEffect::GoAway,
            42,
        )
        .unwrap(),
        Vec::new()
    );
}
