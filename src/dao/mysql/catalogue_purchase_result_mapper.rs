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
