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
