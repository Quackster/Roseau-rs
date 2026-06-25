use super::recorded_player_network::*;

#[test]
fn records_sent_responses_packets_and_close_state() {
    let mut network = RecordedPlayerNetwork::new(12, 30001);
    let response = NettyResponse::with_header("HELLO");

    network.send_response(response);
    network.send_packet("#OK##");
    network.set_server_port(30002);
    network.close();

    assert_eq!(network.connection_id(), 12);
    assert_eq!(network.server_port(), 30002);
    assert_eq!(network.sent_responses().len(), 1);
    assert_eq!(network.sent_responses()[0].header(), "HELLO");
    assert!(network.sent_responses()[0].is_finalised());
    assert_eq!(network.sent_packets(), &["#HELLO##", "#OK##"]);
    assert!(network.is_closed());
}
