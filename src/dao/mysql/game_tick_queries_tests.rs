use super::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

#[test]
fn maps_credit_award_effect_to_update_plan() {
    let plan = GameTickQueries::plan(&GameTickEffect::AwardCredits {
        user_id: 7,
        amount: 10,
        new_balance: 125,
    })
    .unwrap();

    assert_eq!(plan.kind(), SqlExecutionKind::Execute);
    assert_eq!(plan.sql(), "UPDATE users SET credits = ? WHERE id = ?");
    assert_eq!(
        plan.parameters(),
        &[SqlParameter::Integer(125), SqlParameter::Integer(7)]
    );
}

#[test]
fn ignores_non_sql_tick_effects() {
    assert_eq!(
        GameTickQueries::plan(&GameTickEffect::SavePlayer { user_id: 7 }),
        None
    );
    assert_eq!(
        GameTickQueries::plan(&GameTickEffect::ResolveServerIp),
        None
    );
    assert_eq!(
        GameTickQueries::plan(&GameTickEffect::KickAfkUser { user_id: 7 }),
        None
    );
}
