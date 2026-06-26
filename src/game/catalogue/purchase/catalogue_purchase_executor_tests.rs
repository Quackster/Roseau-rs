use std::rc::Rc;

use crate::dao::in_memory::{
    InMemoryCatalogueDao, InMemoryInventoryDao, InMemoryItemDao, InMemoryPlayerDao,
};
use crate::dao::{CreatePlayer, InventoryDao, ItemDao, PlayerDao};
use crate::game::catalogue::{
    CatalogueDeal, CatalogueItem, CataloguePurchaseExecution, CataloguePurchaseExecutor,
    CataloguePurchaseRequest,
};
use crate::game::item::ItemDefinition;
use crate::game::player::PlayerDetails;

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
