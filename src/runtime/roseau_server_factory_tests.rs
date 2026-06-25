use super::*;
use crate::dao::mysql::DatabaseEngine;

fn plan(class_path: &str) -> ServerBootstrapPlan {
    ServerBootstrapPlan::new(
        "127.0.0.1",
        "127.0.0.1",
        37120,
        37119,
        class_path,
        DatabaseEngine::MySql,
        vec![37120, 37119, 37125],
    )
}

#[test]
fn constructs_rust_server_handler_from_bootstrap_plan() {
    let factory = RoseauServerFactory::new();
    let handler = factory
        .construct_handler(&plan(factory.supported_handler_path()))
        .unwrap();

    assert_eq!(handler.ip_address(), "127.0.0.1");
    assert_eq!(handler.ports(), &[37120, 37119, 37125]);
    assert!(handler.message_handler().contains_header("LOGIN"));
}

#[test]
fn builds_listen_plan_from_constructed_handler() {
    let factory = RoseauServerFactory::new();
    let listen_plan = factory
        .listen_plan(&plan(factory.supported_handler_path()))
        .unwrap();

    assert_eq!(listen_plan.bind_ip(), "127.0.0.1");
    assert_eq!(
        listen_plan.bind_addresses(),
        vec![
            "127.0.0.1:37120".to_owned(),
            "127.0.0.1:37119".to_owned(),
            "127.0.0.1:37125".to_owned(),
        ]
    );
}

#[test]
fn constructs_tcp_server_runtime_from_bootstrap_plan() {
    let factory = RoseauServerFactory::new();
    let runtime = factory
        .construct_tcp_runtime(&plan(factory.supported_handler_path()), true, false, 50)
        .unwrap();

    assert_eq!(runtime.server_handler().ip_address(), "127.0.0.1");
    assert_eq!(runtime.server_handler().ports(), &[37120, 37119, 37125]);
    assert_eq!(runtime.acceptor().next_connection_id(), 50);
    assert!(runtime.connections().is_empty());
}

#[test]
fn rejects_unknown_rust_handler_path() {
    let result =
        RoseauServerFactory::new().construct_handler(&plan("roseau::server::OtherHandler"));
    let error = match result {
        Ok(_) => panic!("expected unsupported handler error"),
        Err(error) => error,
    };

    assert_eq!(
        error.to_string(),
        "unsupported server handler: roseau::server::OtherHandler"
    );
}
