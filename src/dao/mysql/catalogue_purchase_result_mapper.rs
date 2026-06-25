use crate::dao::mysql::{CataloguePurchaseQueries, SqlExecutionBatchResult, SqlExecutionPlan};
use crate::dao::DaoError;
use crate::game::catalogue::CataloguePurchasePlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CataloguePurchaseResultMapper;

impl CataloguePurchaseResultMapper {
    pub fn teleporter_pair_link_plans(
        insert_plans: &[SqlExecutionPlan],
        insert_results: SqlExecutionBatchResult,
        purchase: &CataloguePurchasePlan,
        buyer_id: i32,
        current_credits: i32,
    ) -> Result<Vec<SqlExecutionPlan>, DaoError> {
        let item_ids = insert_results.i32_insert_ids_for(insert_plans, "item id")?;
        let [first_item_id, second_item_id] = item_ids.as_slice() else {
            return Err(DaoError::new(format!(
                "Expected 2 teleporter item ids, got {}",
                item_ids.len()
            )));
        };

        Ok(CataloguePurchaseQueries::teleporter_pair_link_plans(
            *first_item_id,
            *second_item_id,
            purchase,
            buyer_id,
            current_credits,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionKind, SqlExecutionResult};
    use crate::game::catalogue::{CataloguePurchaseItemPlan, CataloguePurchasePlan};

    fn purchase() -> CataloguePurchasePlan {
        CataloguePurchasePlan::new(5, [CataloguePurchaseItemPlan::new(7, "", true)])
    }

    #[test]
    fn maps_valid_teleporter_insert_results_to_link_plans() {
        let purchase = purchase();
        let insert_plans =
            CataloguePurchaseQueries::teleporter_pair_insert_plans(&purchase, 42).unwrap();
        let insert_results = SqlExecutionBatchResult::new([
            SqlExecutionResult::insert_id(100),
            SqlExecutionResult::insert_id(101),
        ]);

        let plans = CataloguePurchaseResultMapper::teleporter_pair_link_plans(
            &insert_plans,
            insert_results,
            &purchase,
            42,
            20,
        )
        .unwrap();

        assert_eq!(plans.len(), 3);
        assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
        assert_eq!(
            plans[0].sql(),
            "UPDATE items SET extra_data = ? WHERE id = ?"
        );
        assert_eq!(
            plans[1].sql(),
            "UPDATE items SET extra_data = ? WHERE id = ?"
        );
        assert_eq!(plans[2].sql(), "UPDATE users SET credits = ? WHERE id = ?");
    }

    #[test]
    fn rejects_insert_batches_without_two_generated_ids() {
        let purchase = purchase();
        let insert_plans = [
            CataloguePurchaseQueries::teleporter_pair_insert_plans(&purchase, 42)
                .unwrap()
                .remove(0),
        ];
        let insert_results = SqlExecutionBatchResult::new([SqlExecutionResult::insert_id(100)]);

        assert_eq!(
            CataloguePurchaseResultMapper::teleporter_pair_link_plans(
                &insert_plans,
                insert_results,
                &purchase,
                42,
                20,
            )
            .unwrap_err()
            .message(),
            "Expected 2 teleporter item ids, got 1"
        );
    }

    #[test]
    fn rejects_mismatched_insert_result_kind() {
        let purchase = purchase();
        let insert_plans =
            CataloguePurchaseQueries::teleporter_pair_insert_plans(&purchase, 42).unwrap();
        let insert_results = SqlExecutionBatchResult::new([
            SqlExecutionResult::insert_id(100),
            SqlExecutionResult::affected_rows(1),
        ]);

        assert_eq!(
            CataloguePurchaseResultMapper::teleporter_pair_link_plans(
                &insert_plans,
                insert_results,
                &purchase,
                42,
                20,
            )
            .unwrap_err()
            .message(),
            "SQL execution batch result 1 failed validation: SQL execution kind InsertReturningId returned affected rows result"
        );
    }
}
