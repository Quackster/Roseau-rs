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
mod tests {
    use super::*;

    fn item() -> CatalogueItem {
        CatalogueItem::new("chair", 7, 5)
    }

    fn definition(flags: &str) -> ItemDefinition {
        ItemDefinition::new(7, "chair", "red", 1, 1, 1.0, flags, "Chair", "", "")
    }

    #[test]
    fn rejects_item_purchase_when_credits_are_insufficient() {
        assert_eq!(
            CataloguePurchasePlan::for_item(&item(), &definition("SIF"), "chair", 4),
            None
        );
    }

    #[test]
    fn plans_basic_item_purchase_creation_and_cost() {
        let plan =
            CataloguePurchasePlan::for_item(&item(), &definition("SIF"), "chair", 5).unwrap();

        assert_eq!(plan.cost(), 5);
        assert_eq!(plan.items().len(), 1);
        assert_eq!(plan.items()[0].definition_id(), 7);
        assert_eq!(plan.items()[0].extra_data(), "");
        assert!(!plan.items()[0].is_teleporter_pair());
    }

    #[test]
    fn plans_decoration_extra_data_from_call_payload() {
        let plan = CataloguePurchasePlan::for_item(&item(), &definition("SIFV"), "chair L red", 5)
            .unwrap();

        assert_eq!(plan.items()[0].extra_data(), "L");
    }

    #[test]
    fn preserves_java_literal_space_split_for_decoration_purchase_payload() {
        let plan = CataloguePurchasePlan::for_item(&item(), &definition("SIFV"), "chair  L red", 5)
            .unwrap();

        assert_eq!(plan.items()[0].extra_data(), "");
    }

    #[test]
    fn plans_post_it_default_extra_data() {
        let plan =
            CataloguePurchasePlan::for_item(&item(), &definition("SIFJ"), "note", 5).unwrap();

        assert_eq!(plan.items()[0].extra_data(), "20");
    }

    #[test]
    fn marks_teleporter_purchase_for_pair_creation() {
        let plan =
            CataloguePurchasePlan::for_item(&item(), &definition("SIFX"), "tele", 5).unwrap();

        assert!(plan.items()[0].is_teleporter_pair());
    }

    #[test]
    fn rejects_deal_purchase_when_credits_are_insufficient() {
        let deal = CatalogueDeal::new("bundle", ["chair"], 6);

        assert_eq!(CataloguePurchasePlan::for_deal(&deal, &[item()], 5), None);
    }

    #[test]
    fn plans_deal_purchase_items_and_cost() {
        let deal = CatalogueDeal::new("bundle", ["chair", "poster"], 6);
        let items = [
            CatalogueItem::new("chair", 7, 5),
            CatalogueItem::new("poster", 8, 2).with_extra_data("red"),
        ];
        let plan = CataloguePurchasePlan::for_deal(&deal, &items, 6).unwrap();

        assert_eq!(plan.cost(), 6);
        assert_eq!(plan.items().len(), 2);
        assert_eq!(plan.items()[0].definition_id(), 7);
        assert_eq!(plan.items()[0].extra_data(), "");
        assert_eq!(plan.items()[1].definition_id(), 8);
        assert_eq!(plan.items()[1].extra_data(), "red");
        assert!(!plan.items()[0].is_teleporter_pair());
        assert!(!plan.items()[1].is_teleporter_pair());
    }
}
