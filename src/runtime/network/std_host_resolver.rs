use std::net::ToSocketAddrs;

use crate::runtime::HostResolver;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StdHostResolver;

impl StdHostResolver {
    pub fn new() -> Self {
        Self
    }
}

impl HostResolver for StdHostResolver {
    fn resolve_host(&self, host: &str) -> Result<String, String> {
        (host, 0)
            .to_socket_addrs()
            .map_err(|error| error.to_string())?
            .next()
            .map(|address| address.ip().to_string())
            .ok_or_else(|| format!("no address found for {host}"))
    }
}
