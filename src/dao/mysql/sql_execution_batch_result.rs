use crate::dao::mysql::{SqlExecutionPlan, SqlExecutionResult};
use crate::dao::DaoError;

#[derive(Debug, Clone, PartialEq)]
pub struct SqlExecutionBatchResult {
    results: Vec<SqlExecutionResult>,
}

impl SqlExecutionBatchResult {
    pub fn new(results: impl IntoIterator<Item = SqlExecutionResult>) -> Self {
        Self {
            results: results.into_iter().collect(),
        }
    }

    pub fn results(&self) -> &[SqlExecutionResult] {
        &self.results
    }

    pub fn into_results(self) -> Vec<SqlExecutionResult> {
        self.results
    }

    pub fn validate_for(
        self,
        plans: &[SqlExecutionPlan],
    ) -> Result<Vec<SqlExecutionResult>, DaoError> {
        if plans.len() != self.results.len() {
            return Err(DaoError::new(format!(
                "SQL execution batch returned {} results for {} plans",
                self.results.len(),
                plans.len()
            )));
        }

        plans
            .iter()
            .zip(self.results)
            .enumerate()
            .map(|(index, (plan, result))| {
                plan.validate_result(result).map_err(|error| {
                    DaoError::new(format!(
                        "SQL execution batch result {index} failed validation: {}",
                        error.message()
                    ))
                })
            })
            .collect()
    }

    pub fn insert_ids_for(self, plans: &[SqlExecutionPlan]) -> Result<Vec<i64>, DaoError> {
        let ids = self
            .validate_for(plans)?
            .into_iter()
            .filter_map(|result| match result {
                SqlExecutionResult::InsertId(id) => Some(id),
                _ => None,
            })
            .collect::<Vec<_>>();

        Ok(ids)
    }

    pub fn i32_insert_ids_for(
        self,
        plans: &[SqlExecutionPlan],
        generated_id_label: &'static str,
    ) -> Result<Vec<i32>, DaoError> {
        self.insert_ids_for(plans)?
            .into_iter()
            .map(|id| {
                i32::try_from(id).map_err(|_| {
                    DaoError::new(format!("Generated {generated_id_label} {id} exceeds i32"))
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlParameter, SqlQuery};

    fn insert_plan() -> SqlExecutionPlan {
        SqlExecutionPlan::insert_returning_id(SqlQuery::new(
            "INSERT INTO items (item_id) VALUES (?)",
            [SqlParameter::Integer(7)],
        ))
    }

    fn execute_plan() -> SqlExecutionPlan {
        SqlExecutionPlan::execute(SqlQuery::new("UPDATE users SET credits = 0", []))
    }

    #[test]
    fn validates_batch_results_against_plans_in_order() {
        let plans = [insert_plan(), execute_plan()];
        let result = SqlExecutionBatchResult::new([
            SqlExecutionResult::insert_id(10),
            SqlExecutionResult::affected_rows(1),
        ]);

        assert_eq!(
            result.validate_for(&plans).unwrap(),
            vec![
                SqlExecutionResult::InsertId(10),
                SqlExecutionResult::AffectedRows(1)
            ]
        );
    }

    #[test]
    fn reports_batch_length_mismatch() {
        let result = SqlExecutionBatchResult::new([SqlExecutionResult::insert_id(10)]);

        assert_eq!(
            result
                .validate_for(&[insert_plan(), execute_plan()])
                .unwrap_err()
                .message(),
            "SQL execution batch returned 1 results for 2 plans"
        );
    }

    #[test]
    fn reports_mismatched_result_position() {
        let result = SqlExecutionBatchResult::new([
            SqlExecutionResult::affected_rows(1),
            SqlExecutionResult::affected_rows(1),
        ]);

        assert_eq!(
            result
                .validate_for(&[insert_plan(), execute_plan()])
                .unwrap_err()
                .message(),
            "SQL execution batch result 0 failed validation: SQL execution kind InsertReturningId returned affected rows result"
        );
    }

    #[test]
    fn extracts_insert_ids_after_validation() {
        let plans = [insert_plan(), insert_plan(), execute_plan()];
        let result = SqlExecutionBatchResult::new([
            SqlExecutionResult::insert_id(10),
            SqlExecutionResult::insert_id(11),
            SqlExecutionResult::affected_rows(1),
        ]);

        assert_eq!(result.insert_ids_for(&plans).unwrap(), vec![10, 11]);
    }

    #[test]
    fn extracts_i32_insert_ids_after_validation() {
        let plans = [insert_plan(), insert_plan()];
        let result = SqlExecutionBatchResult::new([
            SqlExecutionResult::insert_id(10),
            SqlExecutionResult::insert_id(11),
        ]);

        assert_eq!(
            result.i32_insert_ids_for(&plans, "item id").unwrap(),
            vec![10, 11]
        );
    }

    #[test]
    fn rejects_i32_insert_id_overflow() {
        let result =
            SqlExecutionBatchResult::new([SqlExecutionResult::insert_id(i64::from(i32::MAX) + 1)]);

        assert_eq!(
            result
                .i32_insert_ids_for(&[insert_plan()], "item id")
                .unwrap_err()
                .message(),
            "Generated item id 2147483648 exceeds i32"
        );
    }
}
