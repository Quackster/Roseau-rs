use super::*;

#[test]
fn parses_java_channel_local_address_port() {
    let plan = PlayerNetworkPlan::from_local_address(17, "/127.0.0.1:30001").unwrap();

    assert_eq!(plan.connection_id(), 17);
    assert_eq!(plan.server_port(), 30001);
}

#[test]
fn rejects_missing_or_invalid_ports() {
    assert_eq!(PlayerNetworkPlan::from_local_address(1, "/127.0.0.1"), None);
    assert_eq!(
        PlayerNetworkPlan::from_local_address(1, "/127.0.0.1:not-a-port"),
        None
    );
    assert_eq!(
        PlayerNetworkPlan::from_local_address(1, "/127.0.0.1:70000"),
        None
    );
}

#[test]
fn plans_write_and_close_effects() {
    let plan = PlayerNetworkPlan::new(22, 30002);

    assert_eq!(
        plan.send_response(NettyResponse::with_header("HELLO")),
        PlayerNetworkEffect::WriteResponse {
            connection_id: 22,
            packet: "#HELLO##".to_owned(),
        }
    );
    assert_eq!(
        plan.close(),
        PlayerNetworkEffect::CloseConnection { connection_id: 22 }
    );
}
