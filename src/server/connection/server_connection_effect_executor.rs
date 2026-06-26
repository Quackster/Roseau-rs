use crate::messages::{IncomingContext, PendingIncomingCommandBatch};
use crate::protocol::NettyResponse;
use crate::server::{
    PlayerNetworkEffect, ServerConnectionEffect, ServerHandler, SessionLifecycleEffect,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConnectionEffectExecutor {
    connection_logs: Vec<String>,
    packet_logs: Vec<String>,
    network_effects: Vec<PlayerNetworkEffect>,
    session_effects: Vec<SessionLifecycleEffect>,
    pending_incoming_commands: Vec<PendingIncomingCommandBatch>,
}

impl ServerConnectionEffectExecutor {
    pub fn new() -> Self {
        Self {
            connection_logs: Vec::new(),
            packet_logs: Vec::new(),
            network_effects: Vec::new(),
            session_effects: Vec::new(),
            pending_incoming_commands: Vec::new(),
        }
    }

    pub fn apply(
        &mut self,
        handler: &mut ServerHandler,
        context: IncomingContext,
        effect: ServerConnectionEffect,
    ) -> IncomingContext {
        match effect {
            ServerConnectionEffect::SendHello { connection_id } => {
                let mut response = NettyResponse::with_header("HELLO");
                self.network_effects
                    .push(PlayerNetworkEffect::WriteResponse {
                        connection_id,
                        packet: response.get(),
                    });
                context
            }
            ServerConnectionEffect::AddSession { connection_id } => {
                handler.open_connection(connection_id);
                self.session_effects
                    .push(SessionLifecycleEffect::StoreSession { connection_id });
                context
            }
            ServerConnectionEffect::RemoveSession { connection_id } => {
                handler.close_connection(connection_id);
                self.session_effects
                    .push(SessionLifecycleEffect::RemoveSession { connection_id });
                context
            }
            ServerConnectionEffect::DisposePlayer { connection_id } => {
                self.session_effects
                    .push(SessionLifecycleEffect::RemovePlayer { connection_id });
                context
            }
            ServerConnectionEffect::LogConnection { line } => {
                self.connection_logs.push(line);
                context
            }
            ServerConnectionEffect::LogPacket { line } => {
                self.packet_logs.push(line);
                context
            }
            ServerConnectionEffect::DispatchRequest {
                connection_id,
                server_port,
                request,
            } => {
                let mut context = handler.dispatch_request(context, &request);
                let commands = context.take_commands();
                if !commands.is_empty() {
                    self.pending_incoming_commands
                        .push(PendingIncomingCommandBatch::new(
                            connection_id,
                            server_port,
                            commands,
                        ));
                }
                for mut response in context.take_sent() {
                    self.network_effects
                        .push(PlayerNetworkEffect::WriteResponse {
                            connection_id,
                            packet: response.get(),
                        });
                }
                context
            }
            ServerConnectionEffect::CloseConnection { connection_id } => {
                self.network_effects
                    .push(PlayerNetworkEffect::CloseConnection { connection_id });
                context
            }
        }
    }

    pub fn apply_all(
        &mut self,
        handler: &mut ServerHandler,
        context: IncomingContext,
        effects: impl IntoIterator<Item = ServerConnectionEffect>,
    ) -> IncomingContext {
        effects.into_iter().fold(context, |context, effect| {
            self.apply(handler, context, effect)
        })
    }

    pub fn connection_logs(&self) -> &[String] {
        &self.connection_logs
    }

    pub fn packet_logs(&self) -> &[String] {
        &self.packet_logs
    }

    pub fn network_effects(&self) -> &[PlayerNetworkEffect] {
        &self.network_effects
    }

    pub fn session_effects(&self) -> &[SessionLifecycleEffect] {
        &self.session_effects
    }

    pub fn pending_incoming_commands(&self) -> &[PendingIncomingCommandBatch] {
        &self.pending_incoming_commands
    }

    pub fn drain_pending_incoming_commands(&mut self) -> Vec<PendingIncomingCommandBatch> {
        std::mem::take(&mut self.pending_incoming_commands)
    }
}

impl Default for ServerConnectionEffectExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "server_connection_effect_executor_tests.rs"]
mod tests;
