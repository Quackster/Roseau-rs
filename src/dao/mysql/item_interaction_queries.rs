use crate::dao::mysql::{PlayerQueries, SqlExecutionPlan};
use crate::game::item::interactors::ItemInteractionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemInteractionQueries;

impl ItemInteractionQueries {
    pub fn plans(
        effects: &[ItemInteractionEffect],
        user_id: i32,
        current_tickets: i32,
    ) -> Vec<SqlExecutionPlan> {
        let ticket_delta: i32 = effects
            .iter()
            .filter_map(|effect| match effect {
                ItemInteractionEffect::DecrementTickets { amount } => Some(*amount),
                _ => None,
            })
            .sum();

        let should_save_player = effects
            .iter()
            .any(|effect| matches!(effect, ItemInteractionEffect::SavePlayer));

        if should_save_player && ticket_delta != 0 {
            vec![Self::update_tickets_plan(
                user_id,
                current_tickets - ticket_delta,
            )]
        } else {
            Vec::new()
        }
    }

    pub fn update_tickets_plan(user_id: i32, tickets: i32) -> SqlExecutionPlan {
        PlayerQueries::update_tickets(user_id, tickets).execute_plan()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert!(
            ItemInteractionQueries::plans(&[ItemInteractionEffect::SavePlayer], 7, 5).is_empty()
        );
    }
}
