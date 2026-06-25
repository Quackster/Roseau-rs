use crate::game::item::interactors::{ItemInteractionEffect, ItemInteractionEffectNetworkPlan};
use crate::game::item::{Item, ItemDefinition};
use crate::game::player::{PlayerDetails, PlayerManager, PlayerSession};
use crate::messages::outgoing::{ActiveObjectUpdate, UpdateWallItem};
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn manager() -> PlayerManager {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(70, 30000, details(7, "Alice")));
    manager.insert(PlayerSession::new(80, 30000, details(8, "Bob")));
    manager.insert(PlayerSession::new(90, 30000, details(9, "Carol")));
    manager
}

fn item(id: i32, sprite: &str, item_data: &str) -> Item {
    Item::new(
        id,
        300,
        7,
        "1",
        2,
        0.0,
        0,
        ItemDefinition::new(id, sprite, "", 1, 1, 0.0, "SF", "", "", ""),
        item_data,
        None,
    )
    .unwrap()
}

fn wall_item(id: i32, sprite: &str) -> Item {
    Item::new(
        id,
        300,
        7,
        ":w=1,2 l=3,4",
        0,
        0.0,
        0,
        ItemDefinition::new(id, sprite, "", 1, 1, 0.0, "IW", "", "", ""),
        "",
        Some("blue".to_owned()),
    )
    .unwrap()
}

#[test]
fn broadcasts_show_program_for_matching_item_only_to_room_players() {
    let effects = ItemInteractionEffectNetworkPlan::plan(
        &ItemInteractionEffect::ShowProgram {
            item_id: 55,
            program: "open".to_owned(),
        },
        7,
        "Alice",
        3,
        &[7, 8],
        &[item(55, "door", "teleport_a")],
        &manager(),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#SHOWPROGRAM\rteleport_a open##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#SHOWPROGRAM\rteleport_a open##".to_owned(),
            },
        ]
    );
}

#[test]
fn sends_direct_pool_and_ticket_packets_to_acting_player() {
    let effects = ItemInteractionEffectNetworkPlan::plan_all(
        &[
            ItemInteractionEffect::OpenPoolChangeBooth,
            ItemInteractionEffect::SendJumpingPlaceOk,
            ItemInteractionEffect::SendTickets,
        ],
        8,
        "Bob",
        12,
        &[7, 8],
        &[],
        &manager(),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#OPEN_UIMAKOPPI##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#JUMPINGPLACE_OK##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#PH_TICKETS 12##".to_owned(),
            },
        ]
    );
}

#[test]
fn broadcasts_jump_and_teleporter_door_packets() {
    let effects = ItemInteractionEffectNetworkPlan::plan_all(
        &[
            ItemInteractionEffect::SendJumpData {
                username: "Alice".to_owned(),
                data: "2 3 4".to_owned(),
            },
            ItemInteractionEffect::SendDoorOut { item_id: 44 },
            ItemInteractionEffect::SendDoorIn { item_id: 44 },
        ],
        7,
        "Alice",
        0,
        &[7, 8],
        &[item(44, "tele", "tele_a")],
        &manager(),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#JUMPDATA\rAlice\r2 3 4##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#JUMPDATA\rAlice\r2 3 4##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#DOOR_OUT\r000044/Alice##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#DOOR_OUT\r000044/Alice##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#DOOR_IN\r000044/Alice##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#DOOR_IN\r000044/Alice##".to_owned(),
            },
        ]
    );
}

#[test]
fn ignores_effects_for_other_boundaries_and_missing_items() {
    let effects = ItemInteractionEffectNetworkPlan::plan_all(
        &[
            ItemInteractionEffect::SavePlayer,
            ItemInteractionEffect::SetItemCustomData {
                item_id: 1,
                custom_data: "TRUE".to_owned(),
            },
            ItemInteractionEffect::ShowProgram {
                item_id: 99,
                program: "open".to_owned(),
            },
            ItemInteractionEffect::SendDoorOut { item_id: 99 },
        ],
        7,
        "Alice",
        0,
        &[7],
        &[],
        &manager(),
    );

    assert!(effects.is_empty());
}

#[test]
fn broadcasts_floor_and_wall_item_status_updates() {
    let floor_item = item(44, "tele", "tele_a");
    let wall_item = wall_item(45, "poster");
    let effects = ItemInteractionEffectNetworkPlan::plan_all(
        &[
            ItemInteractionEffect::UpdateItemStatus { item_id: 44 },
            ItemInteractionEffect::UpdateItemStatus { item_id: 45 },
        ],
        7,
        "Alice",
        0,
        &[7],
        &[floor_item.clone(), wall_item.clone()],
        &manager(),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: ActiveObjectUpdate::new(Some(floor_item)).compose().get(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: UpdateWallItem::new(wall_item).compose().get(),
            },
        ]
    );
}
