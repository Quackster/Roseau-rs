use std::rc::Rc;

use super::catalogue_incoming_plan::*;
use crate::dao::in_memory::{
    InMemoryCatalogueDao, InMemoryInventoryDao, InMemoryItemDao, InMemoryPlayerDao,
};
use crate::dao::{CreatePlayer, InventoryDao, PlayerDao};
use crate::game::catalogue::{CatalogueDeal, CatalogueItem};
use crate::game::item::ItemDefinition;

fn definition(id: i32, flags: &str, sprite: &str) -> ItemDefinition {
    ItemDefinition::new(id, sprite, "red", 1, 1, 1.0, flags, "Name", "Desc", "")
}

fn create_player(username: &str, credits: i32) -> CreatePlayer {
    CreatePlayer::new(
        username,
        "secret",
        format!("{username}@example.test"),
        "hello",
        "hd=100",
        credits,
        "F",
        "1990-01-01",
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
        "F",
        "",
        "",
        "1990-01-01",
        0,
        "",
        1,
    );
    details
}

fn player_dao(details: &PlayerDetails) -> InMemoryPlayerDao {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&create_player(details.username(), details.credits()))
        .unwrap();
    dao
}

#[test]
fn plans_order_info_from_catalogue_manager() {
    let manager = CatalogueManager::with_items_and_deals(
        [CatalogueItem::new("chair", 5, 10)],
        [CatalogueDeal::new("bundle", ["chair"], 8)],
    );
    let catalogue = InMemoryCatalogueDao::new();
    let item_dao = Rc::new(InMemoryItemDao::new());
    let inventory = InMemoryInventoryDao::shared(Rc::clone(&item_dao));
    let buyer = buyer(25);
    let players = player_dao(&buyer);

    let outcomes = CatalogueIncomingPlan::plan(
        &IncomingExecutionEffect::GetOrderInfo {
            call_id: "/chair alice".to_owned(),
        },
        &manager,
        &catalogue,
        &inventory,
        item_dao.as_ref(),
        &players,
        &buyer,
    )
    .unwrap();

    assert_eq!(
        outcomes,
        vec![CatalogueIncomingOutcome::OrderInfo(
            CatalogueOrderInfoPlan::new("chair", 10)
        )]
    );
}

#[test]
fn plans_normal_purchase_through_catalogue_executor() {
    let manager = CatalogueManager::default();
    let catalogue = InMemoryCatalogueDao::new();
    catalogue.insert_item(CatalogueItem::new("chair", 5, 10));
    let item_dao = Rc::new(InMemoryItemDao::new());
    item_dao.insert_definition(definition(5, "SIF", "chair"));
    let inventory = InMemoryInventoryDao::shared(Rc::clone(&item_dao));
    let buyer = buyer(25);
    let players = player_dao(&buyer);

    let outcomes = CatalogueIncomingPlan::plan(
        &IncomingExecutionEffect::Purchase {
            call_id: "chair alice".to_owned(),
        },
        &manager,
        &catalogue,
        &inventory,
        item_dao.as_ref(),
        &players,
        &buyer,
    )
    .unwrap();

    let [CatalogueIncomingOutcome::Purchase(CataloguePurchaseExecution::Purchased { items, buyer })] =
        outcomes.as_slice()
    else {
        panic!("expected purchase execution");
    };
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].owner_id(), 7);
    assert_eq!(buyer.credits(), 15);
    assert_eq!(inventory.inventory_items(7).unwrap().len(), 1);
}

#[test]
fn plans_ticket_purchase_before_normal_purchase() {
    let manager = CatalogueManager::default();
    let catalogue = InMemoryCatalogueDao::new();
    catalogue.insert_item(CatalogueItem::new("x hyppy bob", 5, 1));
    let item_dao = Rc::new(InMemoryItemDao::new());
    let inventory = InMemoryInventoryDao::shared(Rc::clone(&item_dao));
    let buyer = buyer(25);
    let players = InMemoryPlayerDao::new();
    players.create_player(&create_player("alice", 25)).unwrap();
    players.create_player(&create_player("bob", 5)).unwrap();

    let outcomes = CatalogueIncomingPlan::plan(
        &IncomingExecutionEffect::Purchase {
            call_id: "x hyppy bob".to_owned(),
        },
        &manager,
        &catalogue,
        &inventory,
        item_dao.as_ref(),
        &players,
        &buyer,
    )
    .unwrap();

    assert!(matches!(
        outcomes.as_slice(),
        [CatalogueIncomingOutcome::TicketPurchase(
            CatalogueTicketPurchaseExecution::Purchased { .. }
        )]
    ));
}

#[test]
fn ignores_unrelated_effects_and_unknown_order_info() {
    let manager = CatalogueManager::default();
    let catalogue = InMemoryCatalogueDao::new();
    let item_dao = Rc::new(InMemoryItemDao::new());
    let inventory = InMemoryInventoryDao::shared(Rc::clone(&item_dao));
    let buyer = buyer(25);
    let players = player_dao(&buyer);

    assert!(CatalogueIncomingPlan::plan(
        &IncomingExecutionEffect::GoAway,
        &manager,
        &catalogue,
        &inventory,
        item_dao.as_ref(),
        &players,
        &buyer,
    )
    .unwrap()
    .is_empty());
    assert!(CatalogueIncomingPlan::plan(
        &IncomingExecutionEffect::GetOrderInfo {
            call_id: "missing".to_owned(),
        },
        &manager,
        &catalogue,
        &inventory,
        item_dao.as_ref(),
        &players,
        &buyer,
    )
    .unwrap()
    .is_empty());
}
