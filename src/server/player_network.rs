use crate::protocol::NettyResponse;

pub trait PlayerNetwork {
    fn connection_id(&self) -> i32;
    fn server_port(&self) -> u16;
    fn set_server_port(&mut self, server_port: u16);
    fn send_response(&mut self, response: NettyResponse);
    fn send_packet(&mut self, packet: &str);
    fn close(&mut self);
    fn is_closed(&self) -> bool;
}
