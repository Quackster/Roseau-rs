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
