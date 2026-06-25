use crate::messages::outgoing::{PurchaseAddStripItem, SystemBroadcast};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CataloguePurchaseOutcome {
    AddedStripItem,
    NotEnoughCredits,
}

impl CataloguePurchaseOutcome {
    pub const NOT_ENOUGH_CREDITS_MESSAGE: &'static str =
        "You don't have enough credits to purchase this item!";

    pub fn item_or_deal(available_credits: i32, required_credits: i32) -> Self {
        if available_credits >= required_credits {
            Self::AddedStripItem
        } else {
            Self::NotEnoughCredits
        }
    }

    pub fn purchase_add_strip_item(&self) -> Option<PurchaseAddStripItem> {
        matches!(self, Self::AddedStripItem).then_some(PurchaseAddStripItem)
    }

    pub fn system_broadcast(&self) -> Option<SystemBroadcast> {
        matches!(self, Self::NotEnoughCredits)
            .then(|| SystemBroadcast::new(Self::NOT_ENOUGH_CREDITS_MESSAGE))
    }
}
