use std::collections::HashMap;

use crate::dao::mysql::{
    CatalogueQueries, CatalogueResultMapper, SqlExecutionPlan, SqlExecutionResult, SqlExecutor,
};
use crate::dao::{CatalogueDao, DaoError};
use crate::game::catalogue::{CatalogueDeal, CatalogueItem};

#[derive(Debug)]
pub struct MySqlCatalogueDao<E> {
    executor: E,
}

impl<E> MySqlCatalogueDao<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }
}

impl<E: SqlExecutor> MySqlCatalogueDao<E> {
    fn execute_plan(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        let result = self.executor.execute(plan.clone())?;
        plan.validate_result(result)
    }
}

impl<E: SqlExecutor> CatalogueDao for MySqlCatalogueDao<E> {
    fn buyable_items(&self) -> Result<HashMap<String, CatalogueItem>, DaoError> {
        let result = self.execute_plan(CatalogueQueries::buyable_items().read_plan())?;
        CatalogueResultMapper::buyable_items(result)
    }

    fn item_deals(&self) -> Result<HashMap<String, CatalogueDeal>, DaoError> {
        let result = self.execute_plan(CatalogueQueries::item_deals().read_plan())?;
        CatalogueResultMapper::item_deals(result)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::VecDeque;

    use super::*;
    use crate::dao::mysql::{SqlExecutionKind, SqlRow, SqlValue};

    #[derive(Debug, Default)]
    struct RecordingExecutor {
        plans: RefCell<Vec<SqlExecutionPlan>>,
        results: RefCell<VecDeque<Result<SqlExecutionResult, DaoError>>>,
    }

    impl RecordingExecutor {
        fn push_result(&self, result: SqlExecutionResult) {
            self.results.borrow_mut().push_back(Ok(result));
        }

        fn plans(&self) -> Vec<SqlExecutionPlan> {
            self.plans.borrow().clone()
        }
    }

    impl SqlExecutor for RecordingExecutor {
        fn execute(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
            self.plans.borrow_mut().push(plan);
            self.results
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| Err(DaoError::new("missing queued SQL result")))
        }
    }

    fn catalogue_row(call_id: &str, definition_id: i32, credits: i32) -> SqlRow {
        SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("definition_id", SqlValue::Integer(definition_id)),
            ("call_id", SqlValue::Text(call_id.to_owned())),
            ("credits", SqlValue::Integer(credits)),
        ])
    }

    fn deal_row(call_id: &str, products: &str, cost: i32) -> SqlRow {
        SqlRow::new([
            ("id", SqlValue::Integer(2)),
            ("call_id", SqlValue::Text(call_id.to_owned())),
            ("products", SqlValue::Text(products.to_owned())),
            ("cost", SqlValue::Integer(cost)),
        ])
    }

    #[test]
    fn loads_buyable_items_through_read_plan() {
        let executor = RecordingExecutor::default();
        executor.push_result(SqlExecutionResult::rows([catalogue_row("chair", 5, 10)]));
        let dao = MySqlCatalogueDao::new(executor);

        let items = dao.buyable_items().unwrap();

        assert_eq!(items["chair"].definition_id(), 5);
        assert_eq!(items["chair"].credits(), 10);
        let plans = dao.executor().plans();
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].kind(), SqlExecutionKind::ReadRows);
        assert_eq!(plans[0].sql(), "SELECT * FROM catalogue");
        assert!(plans[0].parameters().is_empty());
    }

    #[test]
    fn loads_item_deals_through_read_plan() {
        let executor = RecordingExecutor::default();
        executor.push_result(SqlExecutionResult::rows([deal_row(
            "bundle",
            "chair|red,table",
            15,
        )]));
        let dao = MySqlCatalogueDao::new(executor);

        let deals = dao.item_deals().unwrap();

        assert_eq!(deals["bundle"].cost(), 15);
        assert_eq!(deals["bundle"].item_calls(), &["chair|red", "table"]);
        let plans = dao.executor().plans();
        assert_eq!(plans[0].kind(), SqlExecutionKind::ReadRows);
        assert_eq!(plans[0].sql(), "SELECT * FROM catalogue_deals");
    }

    #[test]
    fn validates_executor_result_kind_before_mapping() {
        let executor = RecordingExecutor::default();
        executor.push_result(SqlExecutionResult::affected_rows(1));
        let dao = MySqlCatalogueDao::new(executor);

        assert_eq!(
            dao.buyable_items().unwrap_err().message(),
            "SQL execution kind ReadRows returned affected rows result"
        );
    }
}
