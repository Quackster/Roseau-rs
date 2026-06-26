use std::fs;
use std::io::Read;
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::game::moderation::{CallForHelp, ModerationEffect};
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::runtime::{RoseauApplicationRuntime, RoseauBootstrap};
use crate::server::{PlayerNetworkEffect, StdTcpSocketBinder};

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "roseau-rs-application-moderation-network-{name}-{nonce}"
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

#[test]
fn plans_moderation_effect_packets() {
    let (root, bootstrap) = bootstrap_with_config("moderation-effect-network-plan", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let application = RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();

    let effects =
        application.plan_moderation_effect_network_effects(&[ModerationEffect::SendCallForHelp {
            moderator_connection_id: 42,
            call: CallForHelp::new("Lobby", "alice", "help", "2026-06-25 10:00"),
        }]);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet:
                "#CRYFORHELP\rPrivate Room: Lobby @ 2026-06-25 10:00\rurl\rFrom: alice;0;Message: help##"
                    .to_owned(),
        }]
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_moderation_effect_packets_to_active_connections() {
    let (root, bootstrap) = bootstrap_with_config("moderation-effect-network-apply", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 980, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();

    let unapplied =
        application.apply_moderation_effect_network_effects(&[ModerationEffect::SendCallForHelp {
            moderator_connection_id: 980,
            call: CallForHelp::new("Lobby", "alice", "help", "2026-06-25 10:00"),
        }]);
    let expected =
        b"#CRYFORHELP\rPrivate Room: Lobby @ 2026-06-25 10:00\rurl\rFrom: alice;0;Message: help##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert!(unapplied.is_empty());
    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}
