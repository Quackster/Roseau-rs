use crate::server::ServerListenOutcome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoseauStartupRuntimeStatus {
    Ready {
        bind_addresses: Vec<String>,
        active_connections: usize,
    },
    BindFailed {
        bind_addresses: Vec<String>,
        failed_address: String,
    },
}

impl RoseauStartupRuntimeStatus {
    pub fn from_listen_outcome(outcome: &ServerListenOutcome, active_connections: usize) -> Self {
        let bind_addresses = outcome.bind_addresses().to_vec();

        match outcome.failed_address() {
            Some(failed_address) => Self::BindFailed {
                bind_addresses,
                failed_address: failed_address.to_owned(),
            },
            None => Self::Ready {
                bind_addresses,
                active_connections,
            },
        }
    }

    pub fn ready(&self) -> bool {
        matches!(self, Self::Ready { .. })
    }

    pub fn bind_addresses(&self) -> &[String] {
        match self {
            Self::Ready { bind_addresses, .. } | Self::BindFailed { bind_addresses, .. } => {
                bind_addresses
            }
        }
    }

    pub fn failed_address(&self) -> Option<&str> {
        match self {
            Self::Ready { .. } => None,
            Self::BindFailed { failed_address, .. } => Some(failed_address),
        }
    }
}

#[cfg(test)]
#[path = "roseau_startup_runtime_status_tests.rs"]
mod tests;
