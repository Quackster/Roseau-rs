use super::*;
use std::io::Read;
use std::net::TcpStream;
use std::time::Duration;

use crate::server::{
    ServerConnectionHandler, ServerHandler, ServerListenEffectExecutor, ServerListenPlan,
    StdTcpSocketBinder,
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

#[test]
fn accepts_connection_runtime_from_bound_listener() {
    let (binder, address) = bound_binder();
    let mut client = TcpStream::connect(address).unwrap();
    let mut acceptor = TcpConnectionAcceptor::new(40);
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    let mut runtime = acceptor.accept_one(&binder, 0).unwrap();
    let mut server_handler = ServerHandler::new(vec![address.port()], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, false);

    runtime.open(&mut server_handler, &connection_handler);

    let mut bytes = [0; 8];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(runtime.connection_id(), 40);
    assert_eq!(&bytes, b"#HELLO##");
    assert_eq!(acceptor.next_connection_id(), 41);
    assert_eq!(acceptor.accepted_connections(), 1);
}

#[test]
fn records_accept_errors_without_consuming_connection_id() {
    let binder = StdTcpSocketBinder::new();
    let mut acceptor = TcpConnectionAcceptor::new(12);

    let error = acceptor.accept_one(&binder, 0).unwrap_err();

    assert_eq!(error, "listener 0 is not bound");
    assert_eq!(
        acceptor.accept_errors(),
        &["listener 0 is not bound".to_owned()]
    );
    assert_eq!(acceptor.next_connection_id(), 12);
    assert_eq!(acceptor.accepted_connections(), 0);
}

#[test]
fn nonblocking_accept_preserves_connection_id_when_idle() {
    let (binder, _address) = bound_binder();
    let mut acceptor = TcpConnectionAcceptor::new(12);

    assert!(acceptor
        .accept_one_nonblocking(&binder, 0)
        .unwrap()
        .is_none());
    assert_eq!(acceptor.next_connection_id(), 12);
    assert_eq!(acceptor.accepted_connections(), 0);
}
