use std::fs;
use std::io::Read;
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::game::player::{PlayerDetails, PlayerSession};
use crate::game::room::entity::{RoomUser, RoomUserEffect};
use crate::game::room::model::Position;
use crate::game::room::schedulers::SchedulerEffect;
use crate::game::room::{RoomEffect, RoomLeaveEffect};
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::runtime::{RoseauApplicationRuntime, RoseauBootstrap};
use crate::server::{PlayerNetworkEffect, StdTcpSocketBinder};

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("roseau-rs-application-network-{name}-{nonce}"))
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

fn room_user(entity_id: i32, username: &str, x: i32, y: i32) -> RoomUser {
    let mut user = RoomUser::new(entity_id, username, "hd=100", "hello", None::<String>);
    user.set_room_id(42);
    user.set_position(Position::new(x, y, 0.0));
    user
}

#[test]
fn plans_room_effect_packets_from_loaded_player_sessions() {
    let (root, bootstrap) = bootstrap_with_config("room-effect-network-plan", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(70, 30000, player_details(7, "alice")));

    let effects = application.plan_room_effect_network_effects(&[
        RoomEffect::SendDoorbell {
            user_id: 7,
            username: "visitor".to_owned(),
        },
        RoomEffect::SendOwnerPrivileges { user_id: 7 },
    ]);

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#DOORBELL_RINGING\rvisitor##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#YOUAREOWNER##".to_owned(),
            },
        ]
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_room_effect_network_closes_to_active_connections() {
    let (root, bootstrap) = bootstrap_with_config("room-effect-network-apply", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 950, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(950, 30000, player_details(7, "alice")));

    let unapplied =
        application.apply_room_effect_network_effects(&[RoomEffect::KickUser { user_id: 7 }]);

    assert!(unapplied.is_empty());
    assert!(application
        .startup_runtime()
        .tcp_runtime()
        .unwrap()
        .connections()
        .is_empty());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn plans_room_user_effect_packets_from_loaded_player_sessions() {
    let (root, bootstrap) = bootstrap_with_config("room-user-effect-network-plan", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(70, 30000, player_details(7, "Alice")));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(80, 30000, player_details(8, "Bob")));
    let room_users = [room_user(7, "Alice", 0, 0), room_user(8, "Bob", 2, 0)];

    let effects = application.plan_room_user_effect_network_effects(
        &[RoomUserEffect::Chat {
            header: "CHAT".to_owned(),
            username: "Alice".to_owned(),
            message: "hello".to_owned(),
        }],
        7,
        &[7, 8],
        &room_users,
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#CHAT\rAlice hello##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#CHAT\rAlice hello##".to_owned(),
            },
        ]
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_room_user_effect_network_closes_to_active_connections() {
    let (root, bootstrap) = bootstrap_with_config("room-user-effect-network-apply", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 960, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(960, 30000, player_details(7, "Alice")));
    let room_users = [room_user(7, "Alice", 0, 0)];

    let unapplied = application.apply_room_user_effect_network_effects(
        &[RoomUserEffect::Kick],
        7,
        &[7],
        &room_users,
    );

    assert!(unapplied.is_empty());
    assert!(application
        .startup_runtime()
        .tcp_runtime()
        .unwrap()
        .connections()
        .is_empty());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn plans_scheduler_effect_packets_from_loaded_player_sessions() {
    let (root, bootstrap) = bootstrap_with_config("scheduler-effect-network-plan", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(70, 30000, player_details(7, "Alice")));
    let mut alice = room_user(7, "Alice", 0, 0);
    alice.set_status("mv", " 1,2,0", true, -1);

    let effects = application.plan_scheduler_effect_network_effects(
        &[SchedulerEffect::SendStatus(vec![7])],
        &[7],
        &[alice],
    );

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 70,
            packet: "#STATUS \rAlice 0,0,0,0,0/mv 1,2,0/##".to_owned(),
        }]
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_scheduler_effect_network_packets_to_active_connections() {
    let (root, bootstrap) = bootstrap_with_config("scheduler-effect-network-apply", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 970, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(970, 30000, player_details(7, "Alice")));

    let unapplied = application.apply_scheduler_effect_network_effects(
        &[SchedulerEffect::ShowProgram(vec![
            "lamp".to_owned(),
            "setlamp".to_owned(),
            "2".to_owned(),
        ])],
        &[7],
        &[],
    );
    let expected = b"#SHOWPROGRAM\rlamp setlamp 2##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert!(unapplied.is_empty());
    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn plans_room_leave_private_connection_close_from_loaded_player_sessions() {
    let (root, bootstrap) = bootstrap_with_config("room-leave-network-plan", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(71, 37119, player_details(7, "alice")));

    let effects = application.plan_room_leave_network_effects(
        &[RoomLeaveEffect::ClosePrivateRoomConnection { user_id: 7 }],
        &[7],
        37119,
    );

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::CloseConnection { connection_id: 71 }]
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_room_leave_logout_broadcast_to_active_connections() {
    let (root, bootstrap) = bootstrap_with_config("room-leave-network-apply", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 940, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(940, 30000, player_details(8, "bob")));

    let unapplied = application.apply_room_leave_network_effects(
        &[RoomLeaveEffect::BroadcastLogout {
            username: "alice".to_owned(),
        }],
        &[8],
        37119,
    );
    let expected = b"#LOGOUT\ralice##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert!(unapplied.is_empty());
    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}
