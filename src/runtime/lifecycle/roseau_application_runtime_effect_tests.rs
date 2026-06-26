use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::game::item::ItemInteractionRuntimeEffect;
use crate::game::player::{PlayerDetails, PlayerEffect, PlayerSession};
use crate::game::room::model::Position;
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomData, RoomEffect, RoomSummary};
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::runtime::{RoseauApplicationRuntime, RoseauBootstrap};
use crate::server::StdTcpSocketBinder;

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("roseau-rs-application-effects-{name}-{nonce}"))
}

fn main_config(server_port: u16, private_server_port: u16) -> String {
    format!(
            "[Server]\nserver.ip=127.0.0.1\nserver.port={server_port}\nserver.private.port={private_server_port}\nserver.class.path=roseau::server::ServerHandler\n\n[Database]\ntype=mysql\n\n[Logging]\nlog.errors=true\nlog.output=true\nlog.connections=true\nlog.packets=true\n"
        )
}

fn bootstrap_with_config(
    name: &str,
    server_port: u16,
    private_server_port: u16,
) -> (PathBuf, RoseauBootstrap) {
    let root = temp_dir(name);
    fs::create_dir_all(&root).unwrap();
    let main_path = root.join("roseau.properties");
    let hotel_path = root.join("habbohotel.properties");
    fs::write(&main_path, main_config(server_port, private_server_port)).unwrap();
    fs::write(&hotel_path, DEFAULT_HOTEL_CONFIG).unwrap();

    (
        root,
        RoseauBootstrap::new(main_path.to_owned(), hotel_path.to_owned()),
    )
}

fn player_details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn room_summary(id: i32, owner_id: i32, hidden: bool) -> RoomSummary {
    RoomSummary::new(RoomData::new(
        id,
        hidden,
        RoomType::Private,
        owner_id,
        "owner",
        format!("room{id}"),
        0,
        "",
        25,
        "desc",
        "model",
        "class",
        "wall",
        "floor",
        false,
        true,
    ))
}

#[test]
fn applies_item_interaction_runtime_ticket_sync_to_active_players() {
    let (root, bootstrap) = bootstrap_with_config("item-runtime-ticket-sync", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let mut alice = player_details(7, "alice");
    alice.set_tickets(5);
    let mut alice_private = player_details(7, "alice-private");
    alice_private.set_tickets(5);
    let mut bob = player_details(8, "bob");
    bob.set_tickets(5);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(10, 37120, alice));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(11, 37121, alice_private));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(12, 37120, bob));

    let unapplied = application.apply_item_interaction_runtime_effects(&[
        ItemInteractionRuntimeEffect::SyncPlayerTickets {
            user_id: 7,
            tickets: 4,
        },
    ]);

    assert!(unapplied.is_empty());
    assert_eq!(
        application
            .game()
            .player_manager()
            .players()
            .get(&10)
            .unwrap()
            .details()
            .tickets(),
        4
    );
    assert_eq!(
        application
            .game()
            .player_manager()
            .players()
            .get(&11)
            .unwrap()
            .details()
            .tickets(),
        4
    );
    assert_eq!(
        application
            .game()
            .player_manager()
            .players()
            .get(&12)
            .unwrap()
            .details()
            .tickets(),
        5
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn keeps_item_interaction_scheduled_and_room_transfer_effects_unapplied() {
    let (root, bootstrap) = bootstrap_with_config("item-runtime-unapplied", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let effects = [
        ItemInteractionRuntimeEffect::ScheduleEffects {
            user_id: 7,
            delay_ms: 800,
            effects: Vec::new(),
        },
        ItemInteractionRuntimeEffect::LoadRoom {
            user_id: 7,
            room_id: 20,
            position: Position::with_rotation(5, 6, 0.0, 2),
            rotation: 2,
        },
        ItemInteractionRuntimeEffect::LeaveRoom {
            user_id: 7,
            room_id: 10,
        },
    ];

    let unapplied = application.apply_item_interaction_runtime_effects(&effects);

    assert_eq!(unapplied, effects);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_player_room_manager_effects_to_loaded_game_rooms() {
    let (root, bootstrap) = bootstrap_with_config("player-room-manager-effects", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .room_manager_mut()
        .add(room_summary(11, 7, false));
    application
        .game_mut()
        .room_manager_mut()
        .add(room_summary(12, 7, true));
    application
        .game_mut()
        .room_manager_mut()
        .add(room_summary(13, 8, false));

    let removed = application
        .apply_player_room_manager_effects(&[PlayerEffect::DisposeOwnedRooms { user_id: 7 }]);

    assert_eq!(
        removed
            .iter()
            .map(|room| room.data().id())
            .collect::<Vec<_>>(),
        vec![11]
    );
    assert!(application
        .game()
        .room_manager()
        .get_room_by_id(11)
        .is_none());
    assert!(application
        .game()
        .room_manager()
        .get_room_by_id(12)
        .is_some());
    assert!(application
        .game()
        .room_manager()
        .get_room_by_id(13)
        .is_some());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn ignores_non_room_manager_player_effects_at_application_boundary() {
    let (root, bootstrap) = bootstrap_with_config("player-room-manager-ignored", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .room_manager_mut()
        .add(room_summary(11, 7, false));

    let removed = application.apply_player_room_manager_effects(&[
        PlayerEffect::UpdateLastLogin { user_id: 7 },
        PlayerEffect::DisposeInventory { user_id: 7 },
    ]);

    assert!(removed.is_empty());
    assert!(application
        .game()
        .room_manager()
        .get_room_by_id(11)
        .is_some());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn plans_player_room_leave_effects_from_loaded_player_sessions() {
    let (root, bootstrap) = bootstrap_with_config("player-room-leave-plan", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(41, 30001, player_details(7, "alice")));

    let effects = application
        .plan_player_room_leave_effects(&[PlayerEffect::LeaveCurrentRoom { connection_id: 41 }]);

    assert_eq!(
        effects,
        vec![RoomEffect::LeaveRoom {
            user_id: 7,
            hotel_view: false,
        }]
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn ignores_unknown_player_room_leave_connections_at_application_boundary() {
    let (root, bootstrap) = bootstrap_with_config("player-room-leave-plan-ignored", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let application = RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();

    let effects = application.plan_player_room_leave_effects(&[
        PlayerEffect::LeaveCurrentRoom { connection_id: 99 },
        PlayerEffect::DisposeInventory { user_id: 7 },
    ]);

    assert!(effects.is_empty());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_room_manager_effects_to_loaded_game_rooms() {
    let (root, bootstrap) = bootstrap_with_config("room-manager-effects", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .room_manager_mut()
        .add(room_summary(11, 7, false));
    application
        .game_mut()
        .room_manager_mut()
        .add(room_summary(12, 7, false));

    let count = application.apply_room_manager_effects(&[
        RoomEffect::RemoveLoadedRoom { room_id: 99 },
        RoomEffect::RemoveLoadedRoom { room_id: 11 },
    ]);

    assert_eq!(count, 1);
    assert!(application
        .game()
        .room_manager()
        .get_room_by_id(11)
        .is_none());
    assert!(application
        .game()
        .room_manager()
        .get_room_by_id(12)
        .is_some());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn ignores_non_room_manager_room_effects_at_application_boundary() {
    let (root, bootstrap) = bootstrap_with_config("room-manager-effect-ignored", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .room_manager_mut()
        .add(room_summary(11, 7, false));

    let count = application.apply_room_manager_effects(&[
        RoomEffect::ClearRuntimeData,
        RoomEffect::SendOwnerPrivileges { user_id: 7 },
    ]);

    assert_eq!(count, 0);
    assert!(application
        .game()
        .room_manager()
        .get_room_by_id(11)
        .is_some());

    fs::remove_dir_all(root).unwrap();
}
