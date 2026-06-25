use std::collections::HashMap;
use std::fmt::{self, Display};

use super::CatalogueItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogueDeal {
    call_id: String,
    item_calls: Vec<String>,
    cost: i32,
}

impl CatalogueDeal {
    pub fn new(
        call_id: impl Into<String>,
        item_calls: impl IntoIterator<Item = impl Into<String>>,
        cost: i32,
    ) -> Self {
        Self {
            call_id: call_id.into(),
            item_calls: item_calls.into_iter().map(Into::into).collect(),
            cost,
        }
    }

    pub fn call_id(&self) -> &str {
        &self.call_id
    }

    pub fn item_calls(&self) -> &[String] {
        &self.item_calls
    }

    pub fn cost(&self) -> i32 {
        self.cost
    }

    pub fn resolve_items(
        &self,
        catalogue_items: &HashMap<String, CatalogueItem>,
    ) -> Result<Vec<CatalogueItem>, ResolveDealItemError> {
        self.item_calls
            .iter()
            .map(|item_call| {
                let (call_id, extra_data) = item_call
                    .split_once('|')
                    .map_or((item_call.as_str(), None), |(call_id, extra_data)| {
                        (call_id, Some(extra_data))
                    });

                let mut item = catalogue_items
                    .get(call_id)
                    .cloned()
                    .ok_or_else(|| ResolveDealItemError::MissingItem(call_id.to_owned()))?;

                if let Some(extra_data) = extra_data {
                    item.set_extra_data(extra_data);
                }

                Ok(item)
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveDealItemError {
    MissingItem(String),
}

impl Display for ResolveDealItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingItem(call_id) => {
                write!(f, "catalogue deal references missing item '{call_id}'")
            }
        }
    }
}

impl std::error::Error for ResolveDealItemError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_deal_items_and_applies_extra_data() {
        let mut items = HashMap::new();
        items.insert("poster".to_owned(), CatalogueItem::new("poster", 1, 2));
        items.insert("chair".to_owned(), CatalogueItem::new("chair", 2, 5));
        let deal = CatalogueDeal::new("bundle", ["poster|red", "chair"], 6);

        let resolved = deal.resolve_items(&items).unwrap();

        assert_eq!(resolved[0].call_id(), "poster");
        assert_eq!(resolved[0].extra_data(), Some("red"));
        assert_eq!(resolved[1].call_id(), "chair");
        assert_eq!(resolved[1].extra_data(), None);
    }
}
