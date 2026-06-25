use crate::server::ServerListenPlan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerListenOutcome {
    bind_addresses: Vec<String>,
    failed_address: Option<String>,
}

impl ServerListenOutcome {
    pub fn new(
        bind_addresses: impl Into<Vec<String>>,
        failed_address: Option<impl Into<String>>,
    ) -> Self {
        Self {
            bind_addresses: bind_addresses.into(),
            failed_address: failed_address.map(Into::into),
        }
    }

    pub fn success_for_plan(plan: &ServerListenPlan) -> Self {
        Self::new(plan.bind_addresses(), None::<String>)
    }

    pub fn failure_for_plan(plan: &ServerListenPlan, failed_address: impl Into<String>) -> Self {
        Self::new(plan.bind_addresses(), Some(failed_address))
    }

    pub fn bind_addresses(&self) -> &[String] {
        &self.bind_addresses
    }

    pub fn failed_address(&self) -> Option<&str> {
        self.failed_address.as_deref()
    }

    pub fn listened(&self) -> bool {
        self.failed_address.is_none()
    }
}
