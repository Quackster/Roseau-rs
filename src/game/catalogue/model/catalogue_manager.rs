use std::collections::HashMap;

use super::{CatalogueDeal, CatalogueItem, ResolveDealItemError};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CatalogueManager {
    items: HashMap<String, CatalogueItem>,
    deals: HashMap<String, CatalogueDeal>,
}

impl CatalogueManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_items_and_deals(
        items: impl IntoIterator<Item = CatalogueItem>,
        deals: impl IntoIterator<Item = CatalogueDeal>,
    ) -> Self {
        Self {
            items: items
                .into_iter()
                .map(|item| (item.call_id().to_owned(), item))
                .collect(),
            deals: deals
                .into_iter()
                .map(|deal| (deal.call_id().to_owned(), deal))
                .collect(),
        }
    }

    pub fn get_item_by_call(&self, call_id: &str) -> Option<&CatalogueItem> {
        self.items.get(call_id)
    }

    pub fn get_deal_by_call(&self, call_id: &str) -> Option<&CatalogueDeal> {
        self.deals.get(call_id)
    }

    pub fn items(&self) -> &HashMap<String, CatalogueItem> {
        &self.items
    }

    pub fn resolve_deal_items(
        &self,
        call_id: &str,
    ) -> Option<Result<Vec<CatalogueItem>, ResolveDealItemError>> {
        self.get_deal_by_call(call_id)
            .map(|deal| deal.resolve_items(&self.items))
    }
}

#[cfg(test)]
#[path = "catalogue_manager_tests.rs"]
mod tests;
