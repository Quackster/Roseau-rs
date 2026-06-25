use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::time::Duration;

use crate::messages::{IncomingCommand, IncomingContext};
use crate::protocol::DecodeError;
use crate::server::{
    PlayerNetwork, PlayerNetworkEffect, ServerConnectionHandler, ServerHandler,
    TcpConnectionRuntime,
};

fn connected_runtime() -> (TcpConnectionRuntime, TcpStream) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let client = TcpStream::connect(address).unwrap();
    let (server_stream, _) = listener.accept().unwrap();

    (TcpConnectionRuntime::from_stream(15, server_stream), client)
}

#[test]
fn opens_tcp_connection_and_writes_hello_packet() {
    let (mut runtime, mut client) = connected_runtime();
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(true, false);
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    runtime.open(&mut server_handler, &connection_handler);

    let mut bytes = [0; 8];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(&bytes, b"#HELLO##");
    assert!(server_handler.session_manager().has_session(15));
    assert_eq!(runtime.connection_id(), 15);
    assert_eq!(
        runtime.effect_executor().connection_logs(),
        &["[15] Connection from 127.0.0.1".to_owned()]
    );
}

#[test]
fn dispatches_complete_tcp_payloads_through_connection_driver() {
    let (runtime, _client) = connected_runtime();
    let mut runtime = runtime.with_context(IncomingContext::new().in_room(true));
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, true);

    runtime
        .read_bytes(b"0010CHAT hello", &mut server_handler, &connection_handler)
        .unwrap();

    assert_eq!(
        runtime.context().commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::Talk {
                mode: "CHAT".to_owned(),
                message: "hello".to_owned(),
            }
        ]
    );
    assert_eq!(
        runtime.effect_executor().packet_logs(),
        &["[15] Received: CHAT hello".to_owned()]
    );
}

#[test]
fn closes_tcp_network_on_decode_error() {
    let (mut runtime, _client) = connected_runtime();
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, false);

    let error = runtime
        .read_bytes(b"BAD!CHAT hello", &mut server_handler, &connection_handler)
        .unwrap_err();

    assert_eq!(error, DecodeError::InvalidLength);
    assert!(runtime.network().is_closed());
}

#[test]
fn reads_from_tcp_network_and_dispatches_complete_frames() {
    let (runtime, mut client) = connected_runtime();
    let mut runtime = runtime.with_context(IncomingContext::new().in_room(true));
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, true);
    runtime
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    client.write_all(b"0010CHAT hello").unwrap();
    let bytes_read = runtime
        .read_from_network(64, &mut server_handler, &connection_handler)
        .unwrap();

    assert_eq!(bytes_read, 14);
    assert_eq!(
        runtime.context().commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::Talk {
                mode: "CHAT".to_owned(),
                message: "hello".to_owned(),
            }
        ]
    );
}

#[test]
fn nonblocking_read_reports_idle_without_closing_connection() {
    let (mut runtime, _client) = connected_runtime();
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, false);

    runtime.set_nonblocking(true).unwrap();
    let bytes_read = runtime
        .read_from_network_nonblocking(64, &mut server_handler, &connection_handler)
        .unwrap();

    assert_eq!(bytes_read, None);
    assert!(!runtime.network().is_closed());
}

#[test]
fn eof_from_tcp_network_closes_connection_lifecycle() {
    let (mut runtime, client) = connected_runtime();
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, false);
    runtime.open(&mut server_handler, &connection_handler);
    runtime
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    client.shutdown(Shutdown::Write).unwrap();
    let bytes_read = runtime
        .read_from_network(64, &mut server_handler, &connection_handler)
        .unwrap();

    assert_eq!(bytes_read, 0);
    assert!(!server_handler.session_manager().has_session(15));
}

#[test]
fn close_removes_session_without_replaying_prior_network_effects() {
    let (mut runtime, mut client) = connected_runtime();
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, false);
    client
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();

    runtime.open(&mut server_handler, &connection_handler);
    runtime.close(&mut server_handler, &connection_handler);

    let mut bytes = Vec::new();
    let _ = client.read_to_end(&mut bytes);

    assert_eq!(bytes, b"#HELLO##");
    assert!(!server_handler.session_manager().has_session(15));
    assert!(!runtime.network().is_closed());
}

#[test]
fn skips_network_effects_for_other_connections() {
    let (mut runtime, _client) = connected_runtime();
    let applied =
        runtime.apply_network_effect(PlayerNetworkEffect::CloseConnection { connection_id: 99 });

    assert!(!applied);
    assert!(!runtime.network().is_closed());
    assert!(runtime
        .network_effect_executor()
        .skipped_effects()
        .is_empty());
}

#[test]
fn applies_matching_external_network_effect() {
    let (mut runtime, _client) = connected_runtime();

    let applied =
        runtime.apply_network_effect(PlayerNetworkEffect::CloseConnection { connection_id: 15 });

    assert!(applied);
    assert!(runtime.network().is_closed());
}
