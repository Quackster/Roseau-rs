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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::OutgoingMessage;

    #[test]
    fn resolves_success_when_credits_cover_purchase() {
        let outcome = CataloguePurchaseOutcome::item_or_deal(5, 5);

        assert_eq!(outcome, CataloguePurchaseOutcome::AddedStripItem);
        assert_eq!(
            outcome.purchase_add_strip_item().unwrap().compose().get(),
            "#ADDSTRIPITEM##"
        );
        assert!(outcome.system_broadcast().is_none());
    }

    #[test]
    fn resolves_broadcast_when_credits_are_insufficient() {
        let outcome = CataloguePurchaseOutcome::item_or_deal(4, 5);

        assert_eq!(outcome, CataloguePurchaseOutcome::NotEnoughCredits);
        assert!(outcome.purchase_add_strip_item().is_none());
        assert_eq!(
            outcome.system_broadcast().unwrap().compose().get(),
            "#SYSTEMBROADCAST\rYou don't have enough credits to purchase this item!##"
        );
    }
}
