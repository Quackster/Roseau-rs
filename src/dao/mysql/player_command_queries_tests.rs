use super::player_command_queries::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

#[test]
fn maps_personal_message_effect_to_greeting_update() {
    let plan = PlayerCommandQueries::plan(
        &IncomingExecutionEffect::AssignPersonalMessage {
            message: "hello".to_owned(),
        },
        7,
    )
    .unwrap();

    assert_eq!(plan.kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plan.sql(),
        "UPDATE users SET personal_greeting = ? WHERE id = ?"
    );
    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text("hello".to_owned()),
            SqlParameter::Integer(7),
        ]
    );
}

#[test]
fn maps_pool_figure_effect_to_profile_update() {
    let plan = PlayerCommandQueries::plan(
        &IncomingExecutionEffect::UpdatePoolFigure {
            pool_figure: "ph=1".to_owned(),
        },
        7,
    )
    .unwrap();

    assert_eq!(plan.kind(), SqlExecutionKind::Execute);
    assert_eq!(plan.sql(), "UPDATE users SET pool_figure = ? WHERE id = ?");
    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text("ph=1".to_owned()),
            SqlParameter::Integer(7),
        ]
    );
}

#[test]
fn ignores_non_player_command_effects() {
    assert_eq!(
        PlayerCommandQueries::plan(&IncomingExecutionEffect::GoAway, 7),
        None
    );
}

#[test]
fn maps_find_user_effect_to_player_details_read() {
    let plan = PlayerCommandQueries::read_plan(&IncomingExecutionEffect::FindUser {
        username: "alice".to_owned(),
    })
    .unwrap();

    assert_eq!(plan.kind(), SqlExecutionKind::ReadRows);
    assert_eq!(plan.sql(), "SELECT * FROM users WHERE username = ? LIMIT 1");
    assert_eq!(plan.parameters(), &[SqlParameter::Text("alice".to_owned())]);
}

#[test]
fn ignores_non_player_read_effects() {
    assert_eq!(
        PlayerCommandQueries::read_plan(&IncomingExecutionEffect::RetrieveUserInfo),
        None
    );
}
