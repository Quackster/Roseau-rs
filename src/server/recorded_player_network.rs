use crate::protocol::NettyResponse;
use crate::server::PlayerNetwork;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedPlayerNetwork {
    connection_id: i32,
    server_port: u16,
    closed: bool,
    sent_responses: Vec<NettyResponse>,
    sent_packets: Vec<String>,
}

impl RecordedPlayerNetwork {
    pub fn new(connection_id: i32, server_port: u16) -> Self {
        Self {
            connection_id,
            server_port,
            closed: false,
            sent_responses: Vec::new(),
            sent_packets: Vec::new(),
        }
    }

    pub fn sent_responses(&self) -> &[NettyResponse] {
        &self.sent_responses
    }

    pub fn sent_packets(&self) -> &[String] {
        &self.sent_packets
    }
}

impl PlayerNetwork for RecordedPlayerNetwork {
    fn connection_id(&self) -> i32 {
        self.connection_id
    }

    fn server_port(&self) -> u16 {
        self.server_port
    }

    fn set_server_port(&mut self, server_port: u16) {
        self.server_port = server_port;
    }

    fn send_response(&mut self, mut response: NettyResponse) {
        self.sent_packets.push(response.get());
        self.sent_responses.push(response);
    }

    fn send_packet(&mut self, packet: &str) {
        self.sent_packets.push(packet.to_owned());
    }

    fn close(&mut self) {
        self.closed = true;
    }

    fn is_closed(&self) -> bool {
        self.closed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
