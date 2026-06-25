use super::server_listen_plan::*;

#[test]
fn builds_listen_plan_from_handler_state() {
    let handler = ServerHandler::new(vec![30001, 30002], "127.0.0.1");

    let plan = ServerListenPlan::from_handler(&handler);

    assert_eq!(plan.bind_ip(), "127.0.0.1");
    assert_eq!(plan.ports(), &[30001, 30002]);
    assert_eq!(plan.pipeline(), &["encoder", "decoder", "handler"]);
    assert_eq!(
        plan.bind_addresses(),
        vec!["127.0.0.1:30001".to_owned(), "127.0.0.1:30002".to_owned()]
    );
    assert!(plan.can_listen());
}

#[test]
fn rejects_empty_bind_inputs() {
    assert!(!ServerListenPlan::new("", vec![30001]).can_listen());
    assert!(!ServerListenPlan::new("127.0.0.1", Vec::<u16>::new()).can_listen());
}

#[test]
fn plans_java_netty_listen_setup_order() {
    let plan = ServerListenPlan::new("127.0.0.1", vec![30001, 30002]);

    assert_eq!(
        plan.listen_effects(),
        vec![
            ServerListenEffect::CreateCachedWorkerPools,
            ServerListenEffect::InstallPipelineStage { name: "encoder" },
            ServerListenEffect::InstallPipelineStage { name: "decoder" },
            ServerListenEffect::InstallPipelineStage { name: "handler" },
            ServerListenEffect::BindAddress {
                address: "127.0.0.1:30001".to_owned(),
            },
            ServerListenEffect::BindAddress {
                address: "127.0.0.1:30002".to_owned(),
            },
        ]
    );
}
