use super::item_interaction_queries::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

#[test]
fn maps_saved_ticket_decrement_to_ticket_update_plan() {
    let plans = ItemInteractionQueries::plans(
        &[
            ItemInteractionEffect::DecrementTickets { amount: 1 },
            ItemInteractionEffect::SendTickets,
            ItemInteractionEffect::SavePlayer,
        ],
        7,
        5,
    );

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
    assert_eq!(plans[0].sql(), "UPDATE users SET tickets = ? WHERE id = ?");
    assert_eq!(
        plans[0].parameters(),
        &[SqlParameter::Integer(4), SqlParameter::Integer(7)]
    );
}

#[test]
fn ignores_unsaved_or_non_ticket_interaction_effects() {
    assert!(ItemInteractionQueries::plans(
        &[ItemInteractionEffect::DecrementTickets { amount: 1 }],
        7,
        5,
    )
    .is_empty());
    assert!(ItemInteractionQueries::plans(&[ItemInteractionEffect::SavePlayer], 7, 5).is_empty());
}
