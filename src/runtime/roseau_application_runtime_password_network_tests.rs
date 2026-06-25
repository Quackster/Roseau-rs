use std::fs;
use std::io::Read;
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::dao::mysql::{
    MySqlPlayerPasswordActionExecutionReport, MySqlPlayerPasswordActionReport,
    SqlExecutionBatchResult,
};
use crate::game::commands::CommandEffect;
use crate::game::player::{
    PlayerDetails, PlayerLoginOutcome, PlayerPasswordActionOutcome, PlayerSession,
};
use crate::messages::IncomingExecutionEffect;
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::runtime::{RoseauApplicationRuntime, RoseauBootstrap};
use crate::server::{PlayerNetwork, StdTcpSocketBinder};

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "roseau-rs-application-password-network-{name}-{nonce}"
    ))
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

#[test]
fn applies_password_action_packets_to_active_connections() {
    let (root, bootstrap) = bootstrap_with_config("password-runtime-packet", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 910, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    let report = MySqlPlayerPasswordActionExecutionReport::new(
        MySqlPlayerPasswordActionReport::from_outcomes(
            [PlayerPasswordActionOutcome::Login(
                PlayerLoginOutcome::failed(),
            )],
            910,
            1234,
        ),
        SqlExecutionBatchResult::new([]),
    );

    let unapplied = application.apply_password_action_runtime_plans(&report);
    let expected = b"#ERROR Login incorrect##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert!(unapplied.is_empty());
    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_password_action_duplicate_connection_closes() {
    let (root, bootstrap) = bootstrap_with_config("password-runtime-close", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 920, None).unwrap();
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
        .insert(PlayerSession::new(920, 37120, player_details(7, "alice")));
    let report = MySqlPlayerPasswordActionExecutionReport::new(
        MySqlPlayerPasswordActionReport::from_outcomes(
            [PlayerPasswordActionOutcome::Login(
                PlayerLoginOutcome::authenticated(
                    &player_details(7, "alice"),
                    "secret",
                    false,
                    37120,
                    37120,
                    Some(920),
                ),
            )],
            921,
            1234,
        ),
        SqlExecutionBatchResult::new([]),
    );

    let unapplied = application.apply_password_action_runtime_plans(&report);

    assert!(unapplied.is_empty());
    assert!(application
        .startup_runtime()
        .tcp_runtime()
        .unwrap()
        .connections()[0]
        .network()
        .is_closed());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_direct_incoming_execution_network_plans_to_active_connections() {
    let (root, bootstrap) = bootstrap_with_config("incoming-runtime-packet", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 930, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();

    let unapplied = application.apply_incoming_execution_runtime_plans(
        &[IncomingExecutionEffect::Command(CommandEffect::SendAlert(
            "hello".to_owned(),
        ))],
        930,
    );
    let expected = b"#SYSTEMBROADCAST\rhello##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert!(unapplied.is_empty());
    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}
