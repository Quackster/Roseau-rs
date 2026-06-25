use super::*;
use crate::server::RecordedPlayerNetwork;

#[test]
fn writes_and_closes_matching_network_effects() {
    let mut network = RecordedPlayerNetwork::new(7, 37120);
    let mut executor = PlayerNetworkEffectExecutor::new();

    executor.apply_all(
        &mut network,
        [
            PlayerNetworkEffect::WriteResponse {
                connection_id: 7,
                packet: "#HELLO##".to_owned(),
            },
            PlayerNetworkEffect::CloseConnection { connection_id: 7 },
        ],
    );

    assert_eq!(network.sent_packets(), &["#HELLO##"]);
    assert!(network.is_closed());
    assert!(executor.skipped_effects().is_empty());
}

#[test]
fn skips_effects_for_other_connections() {
    let mut network = RecordedPlayerNetwork::new(7, 37120);
    let mut executor = PlayerNetworkEffectExecutor::new();

    executor.apply(
        &mut network,
        PlayerNetworkEffect::WriteResponse {
            connection_id: 8,
            packet: "#HELLO##".to_owned(),
        },
    );

    assert!(network.sent_packets().is_empty());
    assert_eq!(
        executor.skipped_effects(),
        &[PlayerNetworkEffect::WriteResponse {
            connection_id: 8,
            packet: "#HELLO##".to_owned(),
        }]
    );
}
