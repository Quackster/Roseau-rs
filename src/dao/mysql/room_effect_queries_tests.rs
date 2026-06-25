use super::room_effect_queries::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

#[test]
fn maps_save_rights_effect_to_delete_then_insert_plans() {
    let plans = RoomEffectQueries::plans(&RoomEffect::SaveRights {
        room_id: 42,
        rights: vec![7, 8],
    });

    assert_eq!(plans.len(), 3);
    assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
    assert_eq!(plans[0].sql(), "DELETE FROM room_rights WHERE room_id = ?");
    assert_eq!(plans[0].parameters(), &[SqlParameter::Integer(42)]);
    assert_eq!(
        plans[1].sql(),
        "INSERT INTO room_rights (room_id, user_id) VALUES (?, ?)"
    );
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
fn keeps_empty_rights_as_delete_only() {
    let plans = RoomEffectQueries::save_rights_plans(42, &[]);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].sql(), "DELETE FROM room_rights WHERE room_id = ?");
}

#[test]
fn ignores_non_persistent_room_effects() {
    assert!(RoomEffectQueries::plans(&RoomEffect::ScheduleWalkTicks).is_empty());
    assert!(
        RoomEffectQueries::plans(&RoomEffect::SendControllerPrivileges { user_id: 7 }).is_empty()
    );
}
