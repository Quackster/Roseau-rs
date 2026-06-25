use super::*;
use std::cell::RefCell;
use std::io::Read;
use std::net::TcpStream;
use std::time::Duration;

use crate::config::Config;
use crate::dao::mysql::DatabaseEngine;
use crate::runtime::roseau_bootstrap::{DEFAULT_HOTEL_CONFIG, DEFAULT_MAIN_CONFIG};
use crate::runtime::ServerBootstrapPlan;
use crate::server::{ServerSocketBinder, StdTcpSocketBinder};

fn runtime() -> RoseauRuntime {
    RoseauRuntime::with_random_seed(
        Config::parse(DEFAULT_MAIN_CONFIG).unwrap(),
        Config::parse(DEFAULT_HOTEL_CONFIG).unwrap(),
        42,
    )
    .unwrap()
}

fn startup_plan(port: u16) -> RoseauStartupPlan {
    RoseauStartupPlan::from_server_plan(ServerBootstrapPlan::new(
        "127.0.0.1",
        "127.0.0.1",
        port,
        37119,
        "roseau::server::ServerHandler",
        DatabaseEngine::MySql,
        vec![port],
        vec![],
    ))
    .unwrap()
}

#[test]
fn prepares_listeners_and_runtime_for_successful_startup() {
    let runtime = runtime();
    let binder = StdTcpSocketBinder::new();
    let mut startup =
        RoseauStartupRuntime::prepare(&runtime, startup_plan(0), &binder, 100).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    assert!(startup.listen_outcome().listened());
    assert!(startup.bind_errors().is_empty());
    assert!(startup.tcp_runtime().is_some());
    assert_eq!(
        startup.status(),
        RoseauStartupRuntimeStatus::Ready {
            bind_addresses: vec!["127.0.0.1:0".to_owned()],
            active_connections: 0,
        }
    );

    let connection_id = startup
        .tcp_runtime_mut()
        .unwrap()
        .accept_and_open_one(&binder, 0)
        .unwrap();

    let mut bytes = [0; 8];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(connection_id, 100);
    assert_eq!(&bytes, b"#HELLO##");
    assert_eq!(
        startup.status(),
        RoseauStartupRuntimeStatus::Ready {
            bind_addresses: vec!["127.0.0.1:0".to_owned()],
            active_connections: 1,
        }
    );
}

#[test]
fn runs_bounded_tick_through_prepared_startup_runtime() {
    let runtime = runtime();
    let binder = StdTcpSocketBinder::new();
    let mut startup =
        RoseauStartupRuntime::prepare(&runtime, startup_plan(0), &binder, 100).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    let first_tick = startup.step(&binder, 0, true, 64).unwrap();
    let mut bytes = [0; 8];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(first_tick.accepted_connection_id(), Some(100));
    assert_eq!(first_tick.read_outcomes(), &[]);
    assert_eq!(
        startup.status(),
        RoseauStartupRuntimeStatus::Ready {
            bind_addresses: vec!["127.0.0.1:0".to_owned()],
            active_connections: 1,
        }
    );
}

#[test]
fn exposes_loop_outcome_for_bounded_tick() {
    let runtime = runtime();
    let binder = StdTcpSocketBinder::new();
    let mut startup =
        RoseauStartupRuntime::prepare(&runtime, startup_plan(0), &binder, 100).unwrap();

    let outcome = startup.run_loop_step(&binder);

    assert!(outcome.should_continue());
    assert!(outcome.tick().is_some());
    assert_eq!(outcome.error(), None);
}

#[derive(Debug, Default)]
struct FailingBinder {
    failures: RefCell<Vec<String>>,
}

impl ServerSocketBinder for FailingBinder {
    fn bind(&self, address: &str) -> Result<(), String> {
        self.failures.borrow_mut().push(address.to_owned());
        Err("bind failed".to_owned())
    }
}

#[test]
fn records_bind_failure_without_constructing_tcp_runtime() {
    let runtime = runtime();
    let binder = FailingBinder::default();
    let mut startup =
        RoseauStartupRuntime::prepare(&runtime, startup_plan(37120), &binder, 1).unwrap();

    assert!(!startup.listen_outcome().listened());
    assert_eq!(
        startup.listen_outcome().failed_address(),
        Some("127.0.0.1:37120")
    );
    assert_eq!(
        startup.bind_errors(),
        &[("127.0.0.1:37120".to_owned(), "bind failed".to_owned())]
    );
    assert!(startup.tcp_runtime().is_none());
    assert_eq!(
        startup.status(),
        RoseauStartupRuntimeStatus::BindFailed {
            bind_addresses: vec!["127.0.0.1:37120".to_owned()],
            failed_address: "127.0.0.1:37120".to_owned(),
        }
    );
    assert_eq!(
        startup.startup_log_lines(None),
        vec![
            "Settting up server".to_owned(),
            "Server could not listen on 37120:37120, please double check everything is correct in icarus.properties".to_owned(),
        ]
    );
    assert_eq!(
        startup.step(&binder, 0, true, 64).unwrap_err(),
        RoseauStartupRuntimeError::NotListening
    );
    assert_eq!(
        startup.run_loop_step(&binder),
        RoseauServerLoopOutcome::Stop {
            error: RoseauStartupRuntimeError::NotListening,
        }
    );
}
