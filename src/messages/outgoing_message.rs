use crate::protocol::NettyResponse;

pub trait OutgoingMessage {
    fn write(&self, response: &mut NettyResponse);

    fn compose(&self) -> NettyResponse {
        let mut response = NettyResponse::new();
        self.write(&mut response);
        response
    }
}
