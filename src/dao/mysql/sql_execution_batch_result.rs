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
#[path = "sql_execution_batch_result_tests.rs"]
mod tests;
