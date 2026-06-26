use std::net::TcpStream;

pub trait ServerSocketBinder {
    fn bind(&self, address: &str) -> Result<(), String>;

    fn accept(&self, listener_index: usize) -> Result<TcpStream, String> {
        Err(format!(
            "accept is not supported for listener {listener_index}"
        ))
    }

    fn accept_nonblocking(&self, listener_index: usize) -> Result<Option<TcpStream>, String> {
        Err(format!(
            "nonblocking accept is not supported for listener {listener_index}"
        ))
    }
}
