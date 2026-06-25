use super::*;
use crate::dao::mysql::DatabaseEngine;
use crate::runtime::ServerBootstrapPlan;

#[test]
fn builds_startup_steps_for_numeric_bind_address() {
    let plan = ServerBootstrapPlan::new(
        "127.0.0.1",
        "127.0.0.1",
        37120,
        37119,
        "roseau::server::ServerHandler",
        DatabaseEngine::MySql,
        vec![37120, 37119, 37125],
    );

    let lifecycle = RoseauLifecyclePlan::from_server_plan(&plan);

    assert_eq!(
        lifecycle.steps(),
        &[
            RoseauLifecycleStep::EnsureConfigFiles,
            RoseauLifecycleStep::StartLogging,
            RoseauLifecycleStep::LoadRuntime,
            RoseauLifecycleStep::ValidateDatabaseEngine {
                engine: DatabaseEngine::MySql,
            },
            RoseauLifecycleStep::OpenDatabase {
                engine: DatabaseEngine::MySql,
            },
            RoseauLifecycleStep::LoadGame,
            RoseauLifecycleStep::SetupServer,
            RoseauLifecycleStep::ConstructServerHandler {
                class_path: "roseau::server::ServerHandler".to_owned(),
                bind_ip: "127.0.0.1".to_owned(),
                ports: vec![37120, 37119, 37125],
            },
            RoseauLifecycleStep::Listen {
                bind_ip: "127.0.0.1".to_owned(),
                port: 37120,
            },
        ]
    );
}

#[test]
fn hostname_bind_adds_resolution_step_before_listen() {
    let plan = ServerBootstrapPlan::new(
        "0.0.0.0",
        "localhost",
        37120,
        37119,
        "roseau::server::ServerHandler",
        DatabaseEngine::MySql,
        vec![37120, 37119],
    );

    let lifecycle = RoseauLifecyclePlan::from_server_plan(&plan);

    assert!(lifecycle
        .steps()
        .contains(&RoseauLifecycleStep::ResolveConfiguredHost {
            host: "localhost".to_owned(),
        }));
    assert_eq!(
        lifecycle.steps().last(),
        Some(&RoseauLifecycleStep::Listen {
            bind_ip: "0.0.0.0".to_owned(),
            port: 37120,
        })
    );
}
