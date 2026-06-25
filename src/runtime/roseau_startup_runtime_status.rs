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
mod tests {
    use super::*;
    use crate::server::{ServerListenOutcome, ServerListenPlan};

    #[test]
    fn reports_ready_runtime_status_from_successful_listen() {
        let plan = ServerListenPlan::new("127.0.0.1", vec![37120, 37119]);
        let outcome = ServerListenOutcome::success_for_plan(&plan);
        let status = RoseauStartupRuntimeStatus::from_listen_outcome(&outcome, 2);

        assert!(status.ready());
        assert_eq!(
            status,
            RoseauStartupRuntimeStatus::Ready {
                bind_addresses: vec!["127.0.0.1:37120".to_owned(), "127.0.0.1:37119".to_owned(),],
                active_connections: 2,
            }
        );
        assert_eq!(status.failed_address(), None);
    }

    #[test]
    fn reports_failed_runtime_status_from_failed_listen() {
        let plan = ServerListenPlan::new("127.0.0.1", vec![37120, 37119]);
        let outcome = ServerListenOutcome::failure_for_plan(&plan, "127.0.0.1:37119");
        let status = RoseauStartupRuntimeStatus::from_listen_outcome(&outcome, 0);

        assert!(!status.ready());
        assert_eq!(status.failed_address(), Some("127.0.0.1:37119"));
        assert_eq!(
            status.bind_addresses(),
            &["127.0.0.1:37120".to_owned(), "127.0.0.1:37119".to_owned(),]
        );
    }
}
