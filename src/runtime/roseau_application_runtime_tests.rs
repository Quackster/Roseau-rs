use crate::runtime::{RoseauApplicationRuntime, RoseauBootstrap};
use std::fs;
use std::io::Read;
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::dao::in_memory::InMemoryItemDao;
use crate::game::commands::CommandEffect;
use crate::game::item::ItemDefinition;
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::server::StdTcpSocketBinder;

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("roseau-rs-application-{name}-{nonce}"))
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

fn item_definition(id: i32, sprite: &str) -> ItemDefinition {
    ItemDefinition::new(id, sprite, "red", 1, 1, 1.0, "SFC", "Chair", "", "")
}

#[test]
fn prepares_application_runtime_and_startup_logs() {
    let (root, bootstrap) = bootstrap_with_config("ready", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 500, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    let outcome = application
        .startup_runtime_mut()
        .run_loop_step(&binder, 0, true, 64);
    let mut bytes = [0; 8];
    client.read_exact(&mut bytes).unwrap();

    assert!(application.runtime().logger().output_enabled());
    assert!(application.game().is_loaded());
    assert!(application.game().command_manager().has_command(":about"));
    assert_eq!(
        application
            .game()
            .variables()
            .unwrap()
            .credits_every_amount(),
        10
    );
    assert_eq!(
        application.startup_scheduler_effects(),
        vec![
            crate::game::GameRuntimeSchedulerEffect::CreateWorkerPool { worker_threads: 8 },
            crate::game::GameRuntimeSchedulerEffect::ScheduleFixedRate {
                task: crate::game::GameRuntimeTask::GameTick,
                initial_delay_ms: 0,
                interval_ms: 1_000,
            },
        ]
    );
    let startup_load_report = application.startup_load_report();
    assert!(startup_load_report.variables_loaded());
    assert!(startup_load_report.room_manager_loaded());
    assert!(startup_load_report.item_manager_loaded());
    assert!(startup_load_report.catalogue_manager_loaded());
    assert!(startup_load_report.command_manager_loaded());
    assert_eq!(
        startup_load_report.scheduler_report().fixed_rate_tasks(),
        &[crate::game::GameRuntimeSchedulerEffect::ScheduleFixedRate {
            task: crate::game::GameRuntimeTask::GameTick,
            initial_delay_ms: 0,
            interval_ms: 1_000,
        }]
    );
    assert_eq!(
        application.startup_log_lines(),
        &[
            "Settting up server".to_owned(),
            "Server is listening on 127.0.0.1:0".to_owned(),
        ]
    );
    assert!(application.status().ready());
    assert!(outcome.should_continue());
    assert_eq!(outcome.tick().unwrap().accepted_connection_id(), Some(500));
    assert_eq!(&bytes, b"#HELLO##");

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_command_effects_to_loaded_item_manager() {
    let (root, bootstrap) = bootstrap_with_config("command-effects", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .item_manager_mut()
        .load_definitions([item_definition(1, "chair")]);
    let dao = InMemoryItemDao::new();
    dao.insert_definition(item_definition(2, "table"));

    application
        .apply_command_effects(&dao, &[CommandEffect::ReloadItemDefinitions])
        .unwrap();

    assert!(application
        .game()
        .item_manager()
        .get_definition(1)
        .is_none());
    assert_eq!(
        application
            .game()
            .item_manager()
            .get_definition(2)
            .unwrap()
            .sprite(),
        "table"
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn ignores_non_runtime_command_effects_at_application_boundary() {
    let (root, bootstrap) = bootstrap_with_config("command-effects-ignored", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    application
        .game_mut()
        .item_manager_mut()
        .load_definitions([item_definition(1, "chair")]);
    let dao = InMemoryItemDao::new();
    dao.insert_definition(item_definition(2, "table"));

    application
        .apply_command_effects(
            &dao,
            &[
                CommandEffect::SendAlert("hello".to_owned()),
                CommandEffect::MarkRoomNeedsUpdate,
            ],
        )
        .unwrap();

    assert_eq!(
        application
            .game()
            .item_manager()
            .get_definition(1)
            .unwrap()
            .sprite(),
        "chair"
    );
    assert!(application
        .game()
        .item_manager()
        .get_definition(2)
        .is_none());

    fs::remove_dir_all(root).unwrap();
}
