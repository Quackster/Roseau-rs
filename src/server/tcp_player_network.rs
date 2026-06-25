use std::io::{ErrorKind, Read, Write};
use std::net::{Shutdown, TcpStream};

use crate::protocol::NettyResponse;
use crate::server::{NetworkEncoder, PlayerNetwork};

#[derive(Debug)]
pub struct TcpPlayerNetwork {
    connection_id: i32,
    server_port: u16,
    stream: TcpStream,
    closed: bool,
    last_error: Option<String>,
}

impl TcpPlayerNetwork {
    pub fn new(connection_id: i32, stream: TcpStream) -> Self {
        let server_port = stream
            .local_addr()
            .map(|address| address.port())
            .unwrap_or_default();

        Self {
            connection_id,
            server_port,
            stream,
            closed: false,
            last_error: None,
        }
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn into_stream(self) -> TcpStream {
        self.stream
    }

    pub fn set_read_timeout(&self, timeout: Option<std::time::Duration>) -> Result<(), String> {
        self.stream
            .set_read_timeout(timeout)
            .map_err(|error| error.to_string())
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> Result<(), String> {
        self.stream
            .set_nonblocking(nonblocking)
            .map_err(|error| error.to_string())
    }

    pub fn read_available(&mut self, buffer: &mut [u8]) -> Result<usize, String> {
        self.stream.read(buffer).map_err(|error| {
            self.record_error(&error);
            error.to_string()
        })
    }

    pub fn read_available_nonblocking(
        &mut self,
        buffer: &mut [u8],
    ) -> Result<Option<usize>, String> {
        match self.stream.read(buffer) {
            Ok(bytes_read) => Ok(Some(bytes_read)),
            Err(error) if error.kind() == ErrorKind::WouldBlock => Ok(None),
            Err(error) => {
                self.record_error(&error);
                Err(error.to_string())
            }
        }
    }

    fn record_error(&mut self, error: impl ToString) {
        self.last_error = Some(error.to_string());
    }
}

impl PlayerNetwork for TcpPlayerNetwork {
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
        let bytes = NetworkEncoder::encode_response(&mut response);
        if let Err(error) = self.stream.write_all(&bytes) {
            self.record_error(error);
        }
    }

    fn send_packet(&mut self, packet: &str) {
        let bytes = NetworkEncoder::encode_text(packet);
        if let Err(error) = self.stream.write_all(&bytes) {
            self.record_error(error);
        }
    }

    fn close(&mut self) {
        if let Err(error) = self.stream.shutdown(Shutdown::Both) {
            self.record_error(error);
        }
        self.closed = true;
    }

    fn is_closed(&self) -> bool {
        self.closed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::net::TcpListener;
    use std::time::Duration;

    #[test]
    fn writes_encoded_responses_to_tcp_stream() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let mut client = TcpStream::connect(address).unwrap();
        let (server_stream, _) = listener.accept().unwrap();
        client
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();

        let mut network = TcpPlayerNetwork::new(12, server_stream);
        network.send_response(NettyResponse::with_header("HELLO"));
        network.send_packet("#OK##");

        let mut bytes = [0; 13];
        client.read_exact(&mut bytes).unwrap();

        assert_eq!(&bytes, b"#HELLO###OK##");
        assert_eq!(network.connection_id(), 12);
        assert_eq!(network.server_port(), address.port());
        assert_eq!(network.last_error(), None);
    }

    #[test]
    fn tracks_close_state() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let _client = TcpStream::connect(address).unwrap();
        let (server_stream, _) = listener.accept().unwrap();
        let mut network = TcpPlayerNetwork::new(7, server_stream);

        network.close();

        assert!(network.is_closed());
    }

    #[test]
    fn reads_available_bytes_from_tcp_stream() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let mut client = TcpStream::connect(address).unwrap();
        let (server_stream, _) = listener.accept().unwrap();
        let mut network = TcpPlayerNetwork::new(4, server_stream);

        client.write_all(b"0010CHAT hello").unwrap();

        let mut buffer = [0; 64];
        let bytes_read = network.read_available(&mut buffer).unwrap();

        assert_eq!(&buffer[..bytes_read], b"0010CHAT hello");
        assert_eq!(network.last_error(), None);
    }

    #[test]
    fn reports_nonblocking_idle_without_recording_error() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let _client = TcpStream::connect(address).unwrap();
        let (server_stream, _) = listener.accept().unwrap();
        let mut network = TcpPlayerNetwork::new(4, server_stream);
        let mut buffer = [0; 64];

        network.set_nonblocking(true).unwrap();

        assert_eq!(
            network.read_available_nonblocking(&mut buffer).unwrap(),
            None
        );
        assert_eq!(network.last_error(), None);
    }
}
