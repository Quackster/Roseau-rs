use crate::dao::{CatalogueDao, DaoError, InventoryDao, ItemDao, PlayerDao};
use crate::game::catalogue::{CataloguePurchaseOutcome, CataloguePurchasePlan};
use crate::game::item::Item;
use crate::game::player::PlayerDetails;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CataloguePurchaseExecutor;

impl CataloguePurchaseExecutor {
    pub fn purchase(
        catalogue_dao: &dyn CatalogueDao,
        inventory_dao: &dyn InventoryDao,
        item_dao: &dyn ItemDao,
        player_dao: &dyn PlayerDao,
        request: CataloguePurchaseRequest<'_>,
    ) -> Result<CataloguePurchaseExecution, DaoError> {
        let call_id = normalise_call_id(request.call_id, request.buyer.username());
        let buyable_items = catalogue_dao.buyable_items()?;
        let item_definitions = item_dao.definitions()?;

        if let Some(item) = buyable_items.get(lookup_call_id(&call_id)) {
            let Some(definition) = item_definitions.get(&item.definition_id()) else {
                return Ok(CataloguePurchaseExecution::Ignored);
            };
            let outcome =
                CataloguePurchaseOutcome::item_or_deal(request.buyer.credits(), item.credits());
            if outcome == CataloguePurchaseOutcome::NotEnoughCredits {
                return Ok(CataloguePurchaseExecution::NotEnoughCredits);
            }

            let Some(plan) = CataloguePurchasePlan::for_item(
                item,
                definition,
                &call_id,
                request.buyer.credits(),
            ) else {
                return Ok(CataloguePurchaseExecution::NotEnoughCredits);
            };

            return Self::apply_plan(inventory_dao, item_dao, player_dao, request.buyer, plan);
        }

        if let Some(deal) = catalogue_dao.item_deals()?.get(&call_id) {
            let resolved_items = deal
                .resolve_items(&buyable_items)
                .map_err(|error| DaoError::new(error.to_string()))?;
            let outcome =
                CataloguePurchaseOutcome::item_or_deal(request.buyer.credits(), deal.cost());
            if outcome == CataloguePurchaseOutcome::NotEnoughCredits {
                return Ok(CataloguePurchaseExecution::NotEnoughCredits);
            }

            let Some(plan) =
                CataloguePurchasePlan::for_deal(deal, &resolved_items, request.buyer.credits())
            else {
                return Ok(CataloguePurchaseExecution::NotEnoughCredits);
            };

            return Self::apply_plan(inventory_dao, item_dao, player_dao, request.buyer, plan);
        }

        Ok(CataloguePurchaseExecution::Ignored)
    }

    fn apply_plan(
        inventory_dao: &dyn InventoryDao,
        item_dao: &dyn ItemDao,
        player_dao: &dyn PlayerDao,
        buyer: &PlayerDetails,
        plan: CataloguePurchasePlan,
    ) -> Result<CataloguePurchaseExecution, DaoError> {
        let mut created_items = Vec::new();
        for item_plan in plan.items() {
            let mut item = inventory_dao.new_item(
                item_plan.definition_id(),
                buyer.id(),
                item_plan.extra_data(),
            )?;

            if item_plan.is_teleporter_pair() {
                let mut paired =
                    inventory_dao.new_item(item_plan.definition_id(), buyer.id(), "")?;
                item.set_custom_data(paired.id().to_string());
                paired.set_custom_data(item.id().to_string());
                item_dao.save_item(&item)?;
                item_dao.save_item(&paired)?;
                created_items.push(paired);
            }

            created_items.push(item);
        }

        let mut updated_buyer = buyer.clone();
        updated_buyer.set_credits(buyer.credits() - plan.cost());
        player_dao.update_player(&updated_buyer)?;

        Ok(CataloguePurchaseExecution::Purchased {
            items: created_items,
            buyer: updated_buyer,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CataloguePurchaseRequest<'a> {
    pub call_id: &'a str,
    pub buyer: &'a PlayerDetails,
}

impl<'a> CataloguePurchaseRequest<'a> {
    pub fn new(call_id: &'a str, buyer: &'a PlayerDetails) -> Self {
        Self { call_id, buyer }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CataloguePurchaseExecution {
    Purchased {
        items: Vec<Item>,
        buyer: PlayerDetails,
    },
    NotEnoughCredits,
    Ignored,
}

fn normalise_call_id(call_id: &str, buyer_username: &str) -> String {
    let without_buyer = if call_id.contains("hyppy") {
        call_id.to_owned()
    } else {
        call_id.replace(&format!(" {buyer_username}"), "")
    };

    without_buyer.replace('/', "")
}

fn lookup_call_id(call_id: &str) -> &str {
    if call_id.contains("L ") || call_id.contains("T ") || call_id.contains("juliste ") {
        call_id.split(' ').next().unwrap_or(call_id)
    } else {
        call_id
    }
}
