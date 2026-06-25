use crate::runtime::{
    BootstrapError, RoseauRuntime, RoseauServerLoopOutcome, RoseauStartupPlan,
    RoseauStartupRuntimeError, RoseauStartupRuntimeStatus,
};
use crate::server::{
    PlayerNetworkEffect, ServerListenEffectExecutor, ServerListenOutcome, ServerSocketBinder,
    TcpServerRuntime, TcpServerTickOutcome,
};

pub struct RoseauStartupRuntime {
    startup_plan: RoseauStartupPlan,
    listen_outcome: ServerListenOutcome,
    tcp_runtime: Option<TcpServerRuntime>,
    bind_errors: Vec<(String, String)>,
}

impl RoseauStartupRuntime {
    pub fn prepare<B: ServerSocketBinder>(
        runtime: &RoseauRuntime,
        startup_plan: RoseauStartupPlan,
        binder: &B,
        first_connection_id: i32,
    ) -> Result<Self, BootstrapError> {
        let mut listen_executor = ServerListenEffectExecutor::new();
        let listen_outcome = listen_executor.execute_plan(startup_plan.listen_plan(), binder);
        let bind_errors = listen_executor.bind_errors().to_vec();
        let tcp_runtime = if listen_outcome.listened() {
            Some(runtime.tcp_server_runtime(startup_plan.server_plan(), first_connection_id)?)
        } else {
            None
        };

        Ok(Self {
            startup_plan,
            listen_outcome,
            tcp_runtime,
            bind_errors,
        })
    }

    pub fn startup_plan(&self) -> &RoseauStartupPlan {
        &self.startup_plan
    }

    pub fn listen_outcome(&self) -> &ServerListenOutcome {
        &self.listen_outcome
    }

    pub fn tcp_runtime(&self) -> Option<&TcpServerRuntime> {
        self.tcp_runtime.as_ref()
    }

    pub fn tcp_runtime_mut(&mut self) -> Option<&mut TcpServerRuntime> {
        self.tcp_runtime.as_mut()
    }

    pub fn bind_errors(&self) -> &[(String, String)] {
        &self.bind_errors
    }

    pub fn status(&self) -> RoseauStartupRuntimeStatus {
        let active_connections = self
            .tcp_runtime
            .as_ref()
            .map(|runtime| runtime.connections().len())
            .unwrap_or_default();

        RoseauStartupRuntimeStatus::from_listen_outcome(&self.listen_outcome, active_connections)
    }

    pub fn startup_log_lines(&self, resolved_config_ip: Option<&str>) -> Vec<String> {
        self.startup_plan
            .startup_log_lines(&self.listen_outcome, resolved_config_ip)
    }

    pub fn step<B: ServerSocketBinder>(
        &mut self,
        binder: &B,
        listener_index: usize,
        accept_connection: bool,
        max_bytes: usize,
    ) -> Result<TcpServerTickOutcome, RoseauStartupRuntimeError> {
        let runtime = self
            .tcp_runtime
            .as_mut()
            .ok_or(RoseauStartupRuntimeError::NotListening)?;

        Ok(runtime.step(binder, listener_index, accept_connection, max_bytes))
    }

    pub fn run_loop_step<B: ServerSocketBinder>(
        &mut self,
        binder: &B,
        listener_index: usize,
        accept_connection: bool,
        max_bytes: usize,
    ) -> RoseauServerLoopOutcome {
        RoseauServerLoopOutcome::from_tick_result(self.step(
            binder,
            listener_index,
            accept_connection,
            max_bytes,
        ))
    }

    pub fn apply_network_effects(
        &mut self,
        effects: impl IntoIterator<Item = PlayerNetworkEffect>,
    ) -> Vec<PlayerNetworkEffect> {
        let Some(runtime) = self.tcp_runtime.as_mut() else {
            return effects.into_iter().collect();
        };

        runtime.apply_network_effects(effects)
    }
}

#[cfg(test)]
#[path = "roseau_startup_runtime_tests.rs"]
mod tests;
