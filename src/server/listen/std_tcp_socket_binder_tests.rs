use super::*;
use crate::server::{ServerListenEffectExecutor, ServerListenPlan};

#[test]
fn binds_and_retains_tcp_listener() {
    let binder = StdTcpSocketBinder::new();

    binder.bind("127.0.0.1:0").unwrap();

    assert_eq!(binder.listener_count(), 1);
    assert_ne!(binder.local_addresses().unwrap()[0].port(), 0);
}

#[test]
fn works_with_server_listen_effect_executor() {
    let binder = StdTcpSocketBinder::new();
    let plan = ServerListenPlan::new("127.0.0.1", vec![0]);
    let mut executor = ServerListenEffectExecutor::new();

    let outcome = executor.execute_plan(&plan, &binder);

    assert!(outcome.listened());
    assert_eq!(binder.listener_count(), 1);
    assert!(executor.bind_errors().is_empty());
}

#[test]
fn accepts_stream_from_retained_listener() {
    let binder = StdTcpSocketBinder::new();
    binder.bind("127.0.0.1:0").unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let _client = std::net::TcpStream::connect(address).unwrap();

    let stream = binder.accept(0).unwrap();

    assert_eq!(stream.local_addr().unwrap().port(), address.port());
}

#[test]
fn reports_nonblocking_accept_idle_without_error() {
    let binder = StdTcpSocketBinder::new();
    binder.bind("127.0.0.1:0").unwrap();

    assert!(binder.accept_nonblocking(0).unwrap().is_none());
}
