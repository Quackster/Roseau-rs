use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::dao::mysql::{Storage, StorageConnector};
use crate::dao::DaoError;
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::runtime::{RoseauApplicationRuntime, RoseauBootstrap, RoseauStartupRuntimeStatus};
use crate::server::{ServerSocketBinder, StdTcpSocketBinder};

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("roseau-rs-application-database-{name}-{nonce}"))
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

#[derive(Debug, Default)]
struct RecordingConnector {
    attempted_urls: RefCell<Vec<String>>,
    result: RefCell<Option<Result<(), DaoError>>>,
}

impl RecordingConnector {
    fn succeed() -> Self {
        Self {
            attempted_urls: RefCell::new(Vec::new()),
            result: RefCell::new(Some(Ok(()))),
        }
    }

    fn fail(message: &str) -> Self {
        Self {
            attempted_urls: RefCell::new(Vec::new()),
            result: RefCell::new(Some(Err(DaoError::new(message)))),
        }
    }
}

impl StorageConnector for RecordingConnector {
    fn connect(&self, storage: &Storage) -> Result<(), DaoError> {
        self.attempted_urls
            .borrow_mut()
            .push(storage.connection_url().to_owned());
        self.result
            .borrow_mut()
            .take()
            .unwrap_or_else(|| Err(DaoError::new("missing connection result")))
    }
}

#[derive(Debug, Default)]
struct FailingBinder {
    attempted_addresses: RefCell<Vec<String>>,
}

impl ServerSocketBinder for FailingBinder {
    fn bind(&self, address: &str) -> Result<(), String> {
        self.attempted_addresses
            .borrow_mut()
            .push(address.to_owned());
        Err("bind failed".to_owned())
    }
}

#[test]
fn prepares_application_runtime_after_database_connection() {
    let (root, bootstrap) = bootstrap_with_config("database-ready", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let connector = RecordingConnector::succeed();

    let report = RoseauApplicationRuntime::prepare_with_database_connector(
        &bootstrap,
        &binder,
        &connector,
        [],
        600,
        None,
    )
    .unwrap();

    assert!(report.ready());
    assert!(report.database_report().connected());
    let application = report.application_runtime().unwrap();
    assert!(application.status().ready());
    assert!(application.game().is_loaded());
    assert!(application.game().command_manager().has_command(":about"));
    assert_eq!(
        connector.attempted_urls.into_inner(),
        vec!["mysql://127.0.0.1:3306/roseau".to_owned()]
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn skips_server_startup_when_database_connection_fails() {
    let (root, bootstrap) = bootstrap_with_config("database-failed", 37120, 37119);
    let binder = StdTcpSocketBinder::new();
    let connector = RecordingConnector::fail("database unavailable");

    let report = RoseauApplicationRuntime::prepare_with_database_connector(
        &bootstrap,
        &binder,
        &connector,
        [],
        1,
        None,
    )
    .unwrap();

    assert!(!report.ready());
    assert_eq!(
        report.database_report().error(),
        Some("database unavailable")
    );
    assert!(report.application_runtime().is_none());
    assert_eq!(binder.listener_count(), 0);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn prepare_report_is_not_ready_when_listener_bind_fails() {
    let (root, bootstrap) = bootstrap_with_config("database-ready-bind-failed", 37120, 37119);
    let binder = FailingBinder::default();
    let connector = RecordingConnector::succeed();

    let report = RoseauApplicationRuntime::prepare_with_database_connector(
        &bootstrap,
        &binder,
        &connector,
        [],
        1,
        None,
    )
    .unwrap();

    assert!(report.database_report().connected());
    assert!(!report.ready());
    let readiness = report.readiness();
    assert!(readiness.database_connected());
    assert!(readiness.game_load_readiness().unwrap().ready());
    assert!(!readiness.startup_status().unwrap().ready());
    assert_eq!(
        readiness.startup_status().unwrap().failed_address(),
        Some("127.0.0.1:37120")
    );
    assert!(report.application_runtime().is_some());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn keeps_application_runtime_with_bind_failure_status() {
    let (root, bootstrap) = bootstrap_with_config("bind-failed", 37120, 37119);
    let binder = FailingBinder::default();

    let application = RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();

    assert_eq!(
            application.startup_log_lines(),
            &[
                "Settting up server".to_owned(),
                "Server could not listen on 37120:37120, please double check everything is correct in icarus.properties".to_owned(),
            ]
        );
    assert_eq!(
        application.status(),
        RoseauStartupRuntimeStatus::BindFailed {
            bind_addresses: vec!["127.0.0.1:37120".to_owned(), "127.0.0.1:37119".to_owned(),],
            failed_address: "127.0.0.1:37120".to_owned(),
        }
    );
    assert_eq!(
        binder.attempted_addresses.into_inner(),
        vec!["127.0.0.1:37120".to_owned()]
    );

    fs::remove_dir_all(root).unwrap();
}
