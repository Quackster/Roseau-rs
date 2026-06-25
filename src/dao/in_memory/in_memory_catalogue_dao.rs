use std::cell::RefCell;
use std::collections::HashMap;

use crate::dao::{CatalogueDao, DaoError};
use crate::game::catalogue::{CatalogueDeal, CatalogueItem};

#[derive(Debug, Default)]
pub struct InMemoryCatalogueDao {
    buyable_items: RefCell<HashMap<String, CatalogueItem>>,
    item_deals: RefCell<HashMap<String, CatalogueDeal>>,
}

impl InMemoryCatalogueDao {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_item(&self, item: CatalogueItem) {
        self.buyable_items
            .borrow_mut()
            .insert(item.call_id().to_owned(), item);
    }

    pub fn insert_deal(&self, deal: CatalogueDeal) {
        self.item_deals
            .borrow_mut()
            .insert(deal.call_id().to_owned(), deal);
    }

    pub fn is_empty(&self) -> bool {
        self.buyable_items.borrow().is_empty() && self.item_deals.borrow().is_empty()
    }
}

impl CatalogueDao for InMemoryCatalogueDao {
    fn buyable_items(&self) -> Result<HashMap<String, CatalogueItem>, DaoError> {
        Ok(self.buyable_items.borrow().clone())
    }

    fn item_deals(&self) -> Result<HashMap<String, CatalogueDeal>, DaoError> {
        Ok(self.item_deals.borrow().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_catalogue_items_and_deals_by_call_id() {
        let dao = InMemoryCatalogueDao::new();
        dao.insert_item(CatalogueItem::new("chair", 5, 10));
        dao.insert_deal(CatalogueDeal::new("bundle", ["chair"], 8));

        assert_eq!(
            dao.buyable_items().unwrap().get("chair").unwrap().credits(),
            10
        );
        assert_eq!(dao.item_deals().unwrap().get("bundle").unwrap().cost(), 8);
    }
}
