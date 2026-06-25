use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;

use crate::messages::IncomingCommand;
use crate::server::{
    PlayerNetwork, PlayerNetworkEffect, ServerConnectionHandler, ServerHandler,
    ServerListenEffectExecutor, ServerListenPlan, StdTcpSocketBinder, TcpServerAcceptOutcome,
    TcpServerRuntime, TcpServerStepOutcome,
};

fn bound_binder() -> (StdTcpSocketBinder, std::net::SocketAddr) {
    let binder = StdTcpSocketBinder::new();
    let plan = ServerListenPlan::new("127.0.0.1", vec![0]);
    let mut executor = ServerListenEffectExecutor::new();
    let outcome = executor.execute_plan(&plan, &binder);

    assert!(outcome.listened());

    let address = binder.local_addresses().unwrap()[0];
    (binder, address)
}

fn runtime_for(address: std::net::SocketAddr) -> TcpServerRuntime {
    TcpServerRuntime::with_first_connection_id(
        ServerHandler::new(vec![address.port()], "127.0.0.1"),
        ServerConnectionHandler::new(false, true),
        70,
    )
}

#[test]
fn accepts_opens_and_tracks_tcp_connection_runtime() {
    let (binder, address) = bound_binder();
    let mut client = TcpStream::connect(address).unwrap();
    let mut runtime = runtime_for(address);
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    let connection_id = runtime.accept_and_open_one(&binder, 0).unwrap();

    let mut bytes = [0; 8];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(connection_id, 70);
    assert_eq!(&bytes, b"#HELLO##");
    assert_eq!(runtime.connections().len(), 1);
    assert!(runtime.server_handler().session_manager().has_session(70));
    assert_eq!(runtime.acceptor().next_connection_id(), 71);
}

#[test]
fn reads_active_connection_and_dispatches_messages() {
    let (binder, address) = bound_binder();
    let mut client = TcpStream::connect(address).unwrap();
    let mut runtime = runtime_for(address);

    runtime.accept_and_open_one(&binder, 0).unwrap();
    runtime
        .connection(0)
        .unwrap()
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    client.write_all(b"0012GIVE_TICKETS").unwrap();
    let bytes_read = runtime.read_connection(0, 64).unwrap();

    assert_eq!(bytes_read, Some(16));
    assert_eq!(
        runtime.connection(0).unwrap().context().commands(),
        &[IncomingCommand::SendTickets]
    );
}

#[test]
fn reads_all_active_connections_once() {
    let (binder, address) = bound_binder();
    let mut first_client = TcpStream::connect(address).unwrap();
    let mut second_client = TcpStream::connect(address).unwrap();
    let mut runtime = runtime_for(address);

    runtime.accept_and_open_one(&binder, 0).unwrap();
    runtime.accept_and_open_one(&binder, 0).unwrap();
    for index in 0..2 {
        runtime
            .connection(index)
            .unwrap()
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
    }

    first_client.write_all(b"0012GIVE_TICKETS").unwrap();
    second_client.write_all(b"0012GIVE_TICKETS").unwrap();

    assert_eq!(
        runtime.read_active_connections(64),
        vec![
            TcpServerStepOutcome::Read {
                connection_id: 70,
                bytes_read: 16,
            },
            TcpServerStepOutcome::Read {
                connection_id: 71,
                bytes_read: 16,
            },
        ]
    );
    assert_eq!(
        runtime.connection(0).unwrap().context().commands(),
        &[IncomingCommand::SendTickets]
    );
    assert_eq!(
        runtime.connection(1).unwrap().context().commands(),
        &[IncomingCommand::SendTickets]
    );
}

#[test]
fn reports_idle_active_connections_without_closing_session() {
    let (binder, address) = bound_binder();
    let _client = TcpStream::connect(address).unwrap();
    let mut runtime = runtime_for(address);

    runtime.accept_and_open_one(&binder, 0).unwrap();

    assert_eq!(
        runtime.read_active_connections(64),
        vec![TcpServerStepOutcome::Idle { connection_id: 70 }]
    );
    assert_eq!(runtime.connections().len(), 1);
    assert!(runtime.server_handler().session_manager().has_session(70));
}

#[test]
fn reports_closed_connections_during_bounded_read_step() {
    let (binder, address) = bound_binder();
    let client = TcpStream::connect(address).unwrap();
    let mut runtime = runtime_for(address);

    runtime.accept_and_open_one(&binder, 0).unwrap();
    runtime
        .connection(0)
        .unwrap()
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    client.shutdown(Shutdown::Write).unwrap();

    assert_eq!(
        runtime.read_active_connections(64),
        vec![TcpServerStepOutcome::Closed { connection_id: 70 }]
    );
    assert!(!runtime.server_handler().session_manager().has_session(70));
}

#[test]
fn removes_closed_connections_after_bounded_read_step() {
    let (binder, address) = bound_binder();
    let closed_client = TcpStream::connect(address).unwrap();
    let mut open_client = TcpStream::connect(address).unwrap();
    let mut runtime = runtime_for(address);

    runtime.accept_and_open_one(&binder, 0).unwrap();
    runtime.accept_and_open_one(&binder, 0).unwrap();
    for index in 0..2 {
        runtime
            .connection(index)
            .unwrap()
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
    }

    closed_client.shutdown(Shutdown::Write).unwrap();
    open_client.write_all(b"0012GIVE_TICKETS").unwrap();
    let outcomes = runtime.read_active_connections(64);

    assert_eq!(
        outcomes,
        vec![
            TcpServerStepOutcome::Closed { connection_id: 70 },
            TcpServerStepOutcome::Read {
                connection_id: 71,
                bytes_read: 16,
            },
        ]
    );
    assert_eq!(runtime.remove_closed_connections(&outcomes), vec![70]);
    assert_eq!(runtime.connections().len(), 1);
    assert_eq!(runtime.connection(0).unwrap().connection_id(), 71);
    assert!(!runtime.server_handler().session_manager().has_session(70));
    assert!(runtime.server_handler().session_manager().has_session(71));
}

#[test]
fn runs_bounded_tick_with_accept_read_and_cleanup() {
    let (binder, address) = bound_binder();
    let closed_client = TcpStream::connect(address).unwrap();
    let mut runtime = runtime_for(address);

    let first_tick = runtime.step(&binder, 0, true, 64);
    runtime
        .connection(0)
        .unwrap()
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    closed_client.shutdown(Shutdown::Write).unwrap();
    let second_tick = runtime.step(&binder, 0, false, 64);

    assert_eq!(
        first_tick.accept_outcome(),
        &TcpServerAcceptOutcome::Accepted { connection_id: 70 }
    );
    assert_eq!(first_tick.accepted_connection_id(), Some(70));
    assert_eq!(first_tick.accept_error(), None);
    assert_eq!(first_tick.read_outcomes(), &[]);
    assert_eq!(first_tick.removed_connection_ids(), &[] as &[i32]);
    assert_eq!(
        second_tick.read_outcomes(),
        &[TcpServerStepOutcome::Closed { connection_id: 70 }]
    );
    assert_eq!(second_tick.removed_connection_ids(), &[70]);
    assert!(runtime.connections().is_empty());
}

#[test]
fn bounded_tick_reports_no_pending_accept_without_blocking() {
    let (binder, address) = bound_binder();
    let mut runtime = runtime_for(address);

    let tick = runtime.step(&binder, 0, true, 64);

    assert_eq!(tick.accept_outcome(), &TcpServerAcceptOutcome::Idle);
    assert_eq!(tick.accepted_connection_id(), None);
    assert_eq!(tick.accept_error(), None);
    assert!(tick.read_outcomes().is_empty());
    assert!(runtime.connections().is_empty());
}

#[test]
fn eof_read_closes_session_for_active_connection() {
    let (binder, address) = bound_binder();
    let client = TcpStream::connect(address).unwrap();
    let mut runtime = runtime_for(address);

    runtime.accept_and_open_one(&binder, 0).unwrap();
    runtime
        .connection(0)
        .unwrap()
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    client.shutdown(Shutdown::Write).unwrap();
    let bytes_read = runtime.read_connection(0, 64).unwrap();

    assert_eq!(bytes_read, Some(0));
    assert!(!runtime.server_handler().session_manager().has_session(70));
}

#[test]
fn reports_missing_active_connection() {
    let (_binder, address) = bound_binder();
    let mut runtime = runtime_for(address);

    assert_eq!(
        runtime.read_connection(0, 64).unwrap_err(),
        "connection 0 is not active"
    );
    assert_eq!(
        runtime.close_connection(0).unwrap_err(),
        "connection 0 is not active"
    );
    assert_eq!(
        runtime.remove_connection(0).unwrap_err(),
        "connection 0 is not active"
    );
}

#[test]
fn removes_connection_by_index_without_closing_session() {
    let (binder, address) = bound_binder();
    let _client = TcpStream::connect(address).unwrap();
    let mut runtime = runtime_for(address);

    runtime.accept_and_open_one(&binder, 0).unwrap();

    assert_eq!(runtime.remove_connection(0).unwrap(), 70);
    assert!(runtime.connections().is_empty());
    assert!(runtime.server_handler().session_manager().has_session(70));
}

#[test]
fn applies_external_network_effects_to_active_connections() {
    let (binder, address) = bound_binder();
    let mut client = TcpStream::connect(address).unwrap();
    let mut runtime = runtime_for(address);
    client
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();

    runtime.accept_and_open_one(&binder, 0).unwrap();
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();

    let unapplied = runtime.apply_network_effects([
        PlayerNetworkEffect::CloseConnection { connection_id: 70 },
        PlayerNetworkEffect::CloseConnection { connection_id: 99 },
    ]);

    assert_eq!(
        unapplied,
        vec![PlayerNetworkEffect::CloseConnection { connection_id: 99 }]
    );
    assert!(runtime.connections()[0].network().is_closed());
}
