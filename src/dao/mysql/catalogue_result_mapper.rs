use std::collections::HashMap;

use crate::dao::mysql::entity::{CatalogueDealRow, CatalogueRow};
use crate::dao::mysql::mapper::{catalogue_deal_from_row, catalogue_item_from_row};
use crate::dao::mysql::SqlExecutionResult;
use crate::dao::DaoError;
use crate::game::catalogue::{CatalogueDeal, CatalogueItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogueResultMapper;

impl CatalogueResultMapper {
    pub fn buyable_items(
        result: SqlExecutionResult,
    ) -> Result<HashMap<String, CatalogueItem>, DaoError> {
        Ok(result
            .map_rows(|row| {
                let catalogue_row = CatalogueRow::try_from(row)?;
                let item = catalogue_item_from_row(&catalogue_row);
                Ok((item.call_id().to_owned(), item))
            })?
            .into_iter()
            .collect())
    }

    pub fn item_deals(
        result: SqlExecutionResult,
    ) -> Result<HashMap<String, CatalogueDeal>, DaoError> {
        Ok(result
            .map_rows(|row| {
                let deal_row = CatalogueDealRow::try_from(row)?;
                let deal = catalogue_deal_from_row(&deal_row);
                Ok((deal.call_id().to_owned(), deal))
            })?
            .into_iter()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlRow, SqlValue};

    #[test]
    fn maps_buyable_item_rows_to_call_id_map() {
        let result = SqlExecutionResult::rows([
            SqlRow::new([
                ("id", SqlValue::Integer(1)),
                ("definition_id", SqlValue::Integer(5)),
                ("call_id", SqlValue::Text("chair".to_owned())),
                ("credits", SqlValue::Integer(10)),
            ]),
            SqlRow::new([
                ("id", SqlValue::Integer(2)),
                ("definition_id", SqlValue::Integer(6)),
                ("call_id", SqlValue::Text("table".to_owned())),
                ("credits", SqlValue::Integer(8)),
            ]),
        ]);

        let items = CatalogueResultMapper::buyable_items(result).unwrap();

        assert_eq!(items.len(), 2);
        assert_eq!(items["chair"].definition_id(), 5);
        assert_eq!(items["table"].credits(), 8);
    }

    #[test]
    fn maps_catalogue_deal_rows_to_call_id_map() {
        let result = SqlExecutionResult::rows([SqlRow::new([
            ("id", SqlValue::Integer(3)),
            ("call_id", SqlValue::Text("bundle".to_owned())),
            ("products", SqlValue::Text("chair|red,table".to_owned())),
            ("cost", SqlValue::Integer(15)),
        ])]);

        let deals = CatalogueResultMapper::item_deals(result).unwrap();

        assert_eq!(deals.len(), 1);
        assert_eq!(deals["bundle"].cost(), 15);
        assert_eq!(deals["bundle"].item_calls(), &["chair|red", "table"]);
    }

    #[test]
    fn rejects_non_row_results_and_invalid_columns() {
        assert_eq!(
            CatalogueResultMapper::buyable_items(SqlExecutionResult::affected_rows(1))
                .unwrap_err()
                .message(),
            "SQL execution result contains affected rows, expected read rows"
        );

        let invalid = SqlExecutionResult::rows([SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("definition_id", SqlValue::Integer(5)),
            ("call_id", SqlValue::Text("chair".to_owned())),
        ])]);

        assert_eq!(
            CatalogueResultMapper::buyable_items(invalid)
                .unwrap_err()
                .message(),
            "Missing or invalid SQL column `credits` as i32"
        );
    }
}
