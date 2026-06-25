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
mod tests {
    use super::*;

    #[test]
    fn finds_catalogue_items_and_deals_by_call_id() {
        let item = CatalogueItem::new("chair", 2, 5);
        let deal = CatalogueDeal::new("bundle", ["chair"], 4);
        let manager = CatalogueManager::with_items_and_deals([item], [deal]);

        assert_eq!(manager.get_item_by_call("chair").unwrap().credits(), 5);
        assert_eq!(manager.get_deal_by_call("bundle").unwrap().cost(), 4);
        assert!(manager.get_item_by_call("missing").is_none());
    }

    #[test]
    fn resolves_items_for_known_deal() {
        let item = CatalogueItem::new("chair", 2, 5);
        let deal = CatalogueDeal::new("bundle", ["chair|green"], 4);
        let manager = CatalogueManager::with_items_and_deals([item], [deal]);
        let resolved = manager.resolve_deal_items("bundle").unwrap().unwrap();

        assert_eq!(resolved[0].extra_data(), Some("green"));
    }
}
