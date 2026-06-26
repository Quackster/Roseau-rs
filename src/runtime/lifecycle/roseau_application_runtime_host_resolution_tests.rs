use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::game::GameTickEffect;
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::runtime::{
    HostResolver, RoseauApplicationRuntime, RoseauApplicationTickExecutionReport, RoseauBootstrap,
    RoseauGameTickRuntimeActionPlan,
};
use crate::server::StdTcpSocketBinder;

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "roseau-rs-application-host-resolution-{name}-{nonce}"
    ))
}

fn main_config_with_ip(server_ip: &str, server_port: u16, private_server_port: u16) -> String {
    format!(
            "[Server]\nserver.ip={server_ip}\nserver.port={server_port}\nserver.private.port={private_server_port}\nserver.class.path=roseau::server::ServerHandler\n\n[Database]\ntype=mysql\n\n[Logging]\nlog.errors=true\nlog.output=true\nlog.connections=true\nlog.packets=true\n"
        )
}

fn bootstrap_with_ip_config(
    name: &str,
    server_ip: &str,
    server_port: u16,
    private_server_port: u16,
) -> (PathBuf, RoseauBootstrap) {
    let root = temp_dir(name);
    fs::create_dir_all(&root).unwrap();
    let main_path = root.join("roseau.properties");
    let hotel_path = root.join("habbohotel.properties");
    fs::write(
        &main_path,
        main_config_with_ip(server_ip, server_port, private_server_port),
    )
    .unwrap();
    fs::write(&hotel_path, DEFAULT_HOTEL_CONFIG).unwrap();

    (
        root,
        RoseauBootstrap::new(main_path.to_owned(), hotel_path.to_owned()),
    )
}

#[derive(Debug)]
struct RecordingHostResolver {
    result: Result<String, String>,
}

impl HostResolver for RecordingHostResolver {
    fn resolve_host(&self, _host: &str) -> Result<String, String> {
        self.result.clone()
    }
}

#[test]
fn applies_tick_runtime_host_resolution_to_advertised_server_ip() {
    let (root, bootstrap) =
        bootstrap_with_ip_config("application-host-resolution", "roseau.local", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let report = RoseauApplicationTickExecutionReport::from_database_report(
        crate::dao::mysql::MySqlApplicationTickExecutionReport::new(
            crate::dao::mysql::SqlExecutionBatchResult::new([]),
            [GameTickEffect::ResolveServerIp],
        ),
        "roseau.local",
        application.game().player_manager(),
    );
    let resolver = RecordingHostResolver {
        result: Ok("10.0.0.25".to_owned()),
    };

    let unapplied = application.apply_tick_runtime_plans_with_resolver(&report, &resolver);

    assert!(unapplied.is_empty());
    assert_eq!(application.resolved_config_ip(), Some("10.0.0.25"));
    assert_eq!(application.advertised_server_ip(), "10.0.0.25");

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn keeps_failed_tick_runtime_host_resolution_unapplied() {
    let (root, bootstrap) =
        bootstrap_with_ip_config("application-host-resolution-failed", "roseau.local", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let report = RoseauApplicationTickExecutionReport::from_database_report(
        crate::dao::mysql::MySqlApplicationTickExecutionReport::new(
            crate::dao::mysql::SqlExecutionBatchResult::new([]),
            [GameTickEffect::ResolveServerIp],
        ),
        "roseau.local",
        application.game().player_manager(),
    );
    let resolver = RecordingHostResolver {
        result: Err("not found".to_owned()),
    };

    let unapplied = application.apply_tick_runtime_plans_with_resolver(&report, &resolver);

    assert_eq!(
        unapplied,
        vec![RoseauGameTickRuntimeActionPlan::ResolveConfiguredHost {
            host: "roseau.local".to_owned(),
        }]
    );
    assert_eq!(application.resolved_config_ip(), None);
    assert_eq!(application.advertised_server_ip(), "roseau.local");

    fs::remove_dir_all(root).unwrap();
}
