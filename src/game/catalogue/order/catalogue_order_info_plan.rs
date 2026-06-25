use crate::game::catalogue::CatalogueManager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogueOrderInfoPlan {
    call_id: String,
    credits: i32,
}

impl CatalogueOrderInfoPlan {
    pub fn resolve(
        manager: &CatalogueManager,
        raw_call_id: &str,
        player_name: Option<&str>,
    ) -> Option<Self> {
        let call_id = normalize_call_id(raw_call_id, player_name);
        let catalogue_id = catalogue_lookup_id(&call_id);

        if let Some(deal) = manager.get_deal_by_call(catalogue_id) {
            return Some(Self::new(deal.call_id(), deal.cost()));
        }

        let item = manager.get_item_by_call(catalogue_id)?;
        let order_call_id = decoration_extra_data(&call_id)
            .map(|extra_data| format!("{} {extra_data}", item.call_id()))
            .unwrap_or_else(|| item.call_id().to_owned());

        Some(Self::new(order_call_id, item.credits()))
    }

    pub fn new(call_id: impl Into<String>, credits: i32) -> Self {
        Self {
            call_id: call_id.into(),
            credits,
        }
    }

    pub fn call_id(&self) -> &str {
        &self.call_id
    }

    pub fn credits(&self) -> i32 {
        self.credits
    }
}

fn normalize_call_id(raw_call_id: &str, player_name: Option<&str>) -> String {
    let without_player = player_name
        .map(|name| raw_call_id.replace(&format!(" {name}"), ""))
        .unwrap_or_else(|| raw_call_id.to_owned());

    without_player.replace('/', "")
}

fn catalogue_lookup_id(call_id: &str) -> &str {
    if has_decoration_payload(call_id) {
        call_id.split(' ').next().unwrap_or(call_id)
    } else {
        call_id
    }
}

fn decoration_extra_data(call_id: &str) -> Option<&str> {
    has_decoration_payload(call_id)
        .then(|| call_id.split(' ').nth(1))
        .flatten()
}

fn has_decoration_payload(call_id: &str) -> bool {
    call_id.contains("L ") || call_id.contains("T ") || call_id.contains("juliste ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::catalogue::{CatalogueDeal, CatalogueItem};

    fn manager() -> CatalogueManager {
        CatalogueManager::with_items_and_deals(
            [
                CatalogueItem::new("chair", 2, 5),
                CatalogueItem::new("poster", 3, 4),
            ],
            [CatalogueDeal::new("bundle", ["chair", "poster"], 7)],
        )
    }

    #[test]
    fn resolves_deal_order_info_before_item_order_info() {
        let plan = CatalogueOrderInfoPlan::resolve(&manager(), "bundle", None).unwrap();

        assert_eq!(plan.call_id(), "bundle");
        assert_eq!(plan.credits(), 7);
    }

    #[test]
    fn resolves_item_order_info_and_strips_player_name() {
        let plan =
            CatalogueOrderInfoPlan::resolve(&manager(), "/chair alice", Some("alice")).unwrap();

        assert_eq!(plan.call_id(), "chair");
        assert_eq!(plan.credits(), 5);
    }

    #[test]
    fn preserves_decoration_extra_data_in_order_name() {
        let plan = CatalogueOrderInfoPlan::resolve(&manager(), "poster L red", None).unwrap();

        assert_eq!(plan.call_id(), "poster L");
        assert_eq!(plan.credits(), 4);
    }

    #[test]
    fn preserves_java_literal_space_split_for_decoration_order_info() {
        let plan = CatalogueOrderInfoPlan::resolve(&manager(), "poster  L red", None).unwrap();

        assert_eq!(plan.call_id(), "poster ");
    }

    #[test]
    fn ignores_unknown_order_info_calls() {
        assert_eq!(
            CatalogueOrderInfoPlan::resolve(&manager(), "missing", None),
            None
        );
    }
}
