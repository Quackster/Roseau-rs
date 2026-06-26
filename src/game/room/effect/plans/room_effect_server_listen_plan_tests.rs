use super::*;
use crate::server::ServerListenEffect;

#[test]
fn maps_public_room_start_effect_to_server_listen_plan() {
    let plan = RoomEffectServerListenPlan::plan(
        &RoomEffect::StartPublicServer {
            room_name: "lido".to_owned(),
            port: 37122,
        },
        "127.0.0.1",
    )
    .unwrap();

    assert_eq!(plan.bind_ip(), "127.0.0.1");
    assert_eq!(plan.ports(), &[37122]);
    assert_eq!(
        plan.listen_effects(),
        vec![
            ServerListenEffect::CreateCachedWorkerPools,
            ServerListenEffect::InstallPipelineStage { name: "encoder" },
            ServerListenEffect::InstallPipelineStage { name: "decoder" },
            ServerListenEffect::InstallPipelineStage { name: "handler" },
            ServerListenEffect::BindAddress {
                address: "127.0.0.1:37122".to_owned(),
            },
        ]
    );
}

#[test]
fn ignores_invalid_ports_and_non_server_room_effects() {
    let plans = RoomEffectServerListenPlan::plan_all(
        &[
            RoomEffect::StartPublicServer {
                room_name: "bad".to_owned(),
                port: -1,
            },
            RoomEffect::SendOwnerPrivileges { user_id: 7 },
        ],
        "127.0.0.1",
    );

    assert!(plans.is_empty());
}
