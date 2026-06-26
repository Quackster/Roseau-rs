use crate::game::catalogue::{CatalogueDeal, CatalogueItem, CataloguePurchaseItemPlan};
use crate::game::item::ItemDefinition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CataloguePurchasePlan {
    cost: i32,
    items: Vec<CataloguePurchaseItemPlan>,
}

impl CataloguePurchasePlan {
    pub fn new(cost: i32, items: impl IntoIterator<Item = CataloguePurchaseItemPlan>) -> Self {
        Self {
            cost,
            items: items.into_iter().collect(),
        }
    }

    pub fn for_item(
        item: &CatalogueItem,
        definition: &ItemDefinition,
        call_id: &str,
        available_credits: i32,
    ) -> Option<Self> {
        if available_credits < item.credits() {
            return None;
        }

        let extra_data = purchase_extra_data(definition, call_id);
        let teleporter_pair = definition.behaviour().is_teleporter();

        Some(Self {
            cost: item.credits(),
            items: vec![CataloguePurchaseItemPlan::new(
                item.definition_id(),
                extra_data,
                teleporter_pair,
            )],
        })
    }

    pub fn for_deal(
        deal: &CatalogueDeal,
        resolved_items: &[CatalogueItem],
        available_credits: i32,
    ) -> Option<Self> {
        if available_credits < deal.cost() {
            return None;
        }

        Some(Self {
            cost: deal.cost(),
            items: resolved_items
                .iter()
                .map(|item| {
                    CataloguePurchaseItemPlan::new(
                        item.definition_id(),
                        item.extra_data().unwrap_or_default(),
                        false,
                    )
                })
                .collect(),
        })
    }

    pub fn cost(&self) -> i32 {
        self.cost
    }

    pub fn items(&self) -> &[CataloguePurchaseItemPlan] {
        &self.items
    }
}

fn purchase_extra_data(definition: &ItemDefinition, call_id: &str) -> String {
    let behaviour = definition.behaviour();

    if behaviour.is_decoration() || call_id.contains("juliste ") {
        return call_id
            .split(' ')
            .nth(1)
            .map(str::to_owned)
            .unwrap_or_default();
    }

    if behaviour.is_post_it() {
        return "20".to_owned();
    }

    String::new()
}

#[cfg(test)]
#[path = "catalogue_purchase_plan_tests.rs"]
mod tests;
