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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::dao::in_memory::{
        InMemoryCatalogueDao, InMemoryInventoryDao, InMemoryItemDao, InMemoryPlayerDao,
    };
    use crate::dao::{CreatePlayer, InventoryDao, ItemDao, PlayerDao};
    use crate::game::catalogue::{CatalogueDeal, CatalogueItem};
    use crate::game::item::ItemDefinition;

    fn definition(id: i32, flags: &str, sprite: &str, data_class: &str) -> ItemDefinition {
        ItemDefinition::new(
            id, sprite, "red", 1, 1, 1.0, flags, "Name", "Desc", data_class,
        )
    }

    fn buyer(credits: i32) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_full(
            7,
            "alice",
            "hello",
            "hd=100",
            "",
            "alice@example.test",
            1,
            credits,
            "Female",
            "",
            "",
            "08.08.1997",
            0,
            "",
            0,
        );
        details
    }

    fn player_dao(details: &PlayerDetails) -> InMemoryPlayerDao {
        let dao = InMemoryPlayerDao::new();
        dao.create_player(&CreatePlayer::new(
            details.username(),
            details.password(),
            details.email(),
            details.mission(),
            details.figure(),
            details.credits(),
            details.sex(),
            details.birthday(),
        ))
        .unwrap();
        dao
    }

    fn purchase(
        catalogue: &InMemoryCatalogueDao,
        inventory: &InMemoryInventoryDao,
        item_dao: &InMemoryItemDao,
        players: &InMemoryPlayerDao,
        buyer: &PlayerDetails,
        call_id: &str,
    ) -> CataloguePurchaseExecution {
        CataloguePurchaseExecutor::purchase(
            catalogue,
            inventory,
            item_dao,
            players,
            CataloguePurchaseRequest::new(call_id, buyer),
        )
        .unwrap()
    }

    #[test]
    fn creates_item_and_debits_buyer_credits() {
        let catalogue = InMemoryCatalogueDao::new();
        catalogue.insert_item(CatalogueItem::new("chair", 5, 10));
        let item_dao = Rc::new(InMemoryItemDao::new());
        item_dao.insert_definition(definition(5, "SIF", "chair", ""));
        let inventory = InMemoryInventoryDao::shared(Rc::clone(&item_dao));
        let buyer = buyer(25);
        let players = player_dao(&buyer);

        let outcome = purchase(
            &catalogue,
            &inventory,
            &item_dao,
            &players,
            &buyer,
            "chair alice",
        );

        let CataloguePurchaseExecution::Purchased { items, buyer } = outcome else {
            panic!("expected purchase");
        };
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].owner_id(), 7);
        assert_eq!(items[0].custom_data(), Some(""));
        assert_eq!(buyer.credits(), 15);
        assert_eq!(
            players
                .details_by_username("alice")
                .unwrap()
                .unwrap()
                .credits(),
            15
        );
    }

    #[test]
    fn applies_decoration_and_post_it_extra_data() {
        let catalogue = InMemoryCatalogueDao::new();
        catalogue.insert_item(CatalogueItem::new("poster", 6, 1));
        catalogue.insert_item(CatalogueItem::new("note", 7, 1));
        let item_dao = Rc::new(InMemoryItemDao::new());
        item_dao.insert_definition(definition(6, "SIFV", "poster", "WALL"));
        item_dao.insert_definition(definition(7, "SIFJ", "post_it", "NOTE"));
        let inventory = InMemoryInventoryDao::shared(Rc::clone(&item_dao));
        let buyer = buyer(10);
        let players = player_dao(&buyer);

        let poster = purchase(
            &catalogue,
            &inventory,
            &item_dao,
            &players,
            &buyer,
            "poster L red",
        );
        let note_buyer = players.details_by_username("alice").unwrap().unwrap();
        let note = purchase(
            &catalogue,
            &inventory,
            &item_dao,
            &players,
            &note_buyer,
            "note",
        );

        let CataloguePurchaseExecution::Purchased { items: poster, .. } = poster else {
            panic!("expected poster purchase");
        };
        let CataloguePurchaseExecution::Purchased { items: note, .. } = note else {
            panic!("expected note purchase");
        };
        assert_eq!(poster[0].custom_data(), Some("L"));
        assert_eq!(note[0].custom_data(), Some("20"));
    }

    #[test]
    fn creates_cross_linked_teleporter_pair() {
        let catalogue = InMemoryCatalogueDao::new();
        catalogue.insert_item(CatalogueItem::new("tele", 8, 5));
        let item_dao = Rc::new(InMemoryItemDao::new());
        item_dao.insert_definition(definition(8, "SIFX", "teleport", ""));
        let inventory = InMemoryInventoryDao::shared(Rc::clone(&item_dao));
        let buyer = buyer(10);
        let players = player_dao(&buyer);

        let outcome = purchase(&catalogue, &inventory, &item_dao, &players, &buyer, "tele");

        let CataloguePurchaseExecution::Purchased { items, .. } = outcome else {
            panic!("expected teleporter purchase");
        };
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].custom_data(), Some("1"));
        assert_eq!(items[1].custom_data(), Some("2"));
        assert_eq!(
            item_dao.item(items[0].id()).unwrap().unwrap().custom_data(),
            Some("1")
        );
        assert_eq!(
            item_dao.item(items[1].id()).unwrap().unwrap().custom_data(),
            Some("2")
        );
    }

    #[test]
    fn creates_deal_items_and_uses_deal_extra_data() {
        let catalogue = InMemoryCatalogueDao::new();
        catalogue.insert_item(CatalogueItem::new("chair", 5, 3));
        catalogue.insert_item(CatalogueItem::new("poster", 6, 2));
        catalogue.insert_deal(CatalogueDeal::new("bundle", ["chair", "poster|blue"], 4));
        let item_dao = Rc::new(InMemoryItemDao::new());
        item_dao.insert_definition(definition(5, "SIF", "chair", ""));
        item_dao.insert_definition(definition(6, "SIFV", "poster", "WALL"));
        let inventory = InMemoryInventoryDao::shared(Rc::clone(&item_dao));
        let buyer = buyer(10);
        let players = player_dao(&buyer);

        let outcome = purchase(
            &catalogue, &inventory, &item_dao, &players, &buyer, "bundle",
        );

        let CataloguePurchaseExecution::Purchased { items, buyer } = outcome else {
            panic!("expected deal purchase");
        };
        assert_eq!(items.len(), 2);
        assert_eq!(items[1].custom_data(), Some("blue"));
        assert_eq!(buyer.credits(), 6);
    }

    #[test]
    fn rejects_missing_or_insufficient_credit_purchase_without_mutation() {
        let catalogue = InMemoryCatalogueDao::new();
        catalogue.insert_item(CatalogueItem::new("chair", 5, 10));
        let item_dao = Rc::new(InMemoryItemDao::new());
        item_dao.insert_definition(definition(5, "SIF", "chair", ""));
        let inventory = InMemoryInventoryDao::shared(Rc::clone(&item_dao));
        let buyer = buyer(5);
        let players = player_dao(&buyer);

        assert_eq!(
            purchase(&catalogue, &inventory, &item_dao, &players, &buyer, "chair"),
            CataloguePurchaseExecution::NotEnoughCredits
        );
        assert_eq!(
            purchase(&catalogue, &inventory, &item_dao, &players, &buyer, "missing"),
            CataloguePurchaseExecution::Ignored
        );
        assert!(inventory.inventory_items(7).unwrap().is_empty());
        assert_eq!(
            players
                .details_by_username("alice")
                .unwrap()
                .unwrap()
                .credits(),
            5
        );
    }
}
