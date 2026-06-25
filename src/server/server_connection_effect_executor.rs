use crate::messages::IncomingContext;
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
}

impl ServerConnectionEffectExecutor {
    pub fn new() -> Self {
        Self {
            connection_logs: Vec::new(),
            packet_logs: Vec::new(),
            network_effects: Vec::new(),
            session_effects: Vec::new(),
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
            ServerConnectionEffect::DispatchRequest { request, .. } => {
                handler.dispatch_request(context, &request)
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
}

impl Default for ServerConnectionEffectExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::IncomingCommand;
    use crate::protocol::NettyRequest;
    use crate::server::ServerConnectionHandler;

    #[test]
    fn applies_open_and_close_effects_to_server_state() {
        let connection_handler = ServerConnectionHandler::new(true, false);
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let mut executor = ServerConnectionEffectExecutor::new();

        executor.apply_all(
            &mut server_handler,
            IncomingContext::new(),
            connection_handler.channel_open(7, "/127.0.0.1:37120"),
        );

        assert!(server_handler.session_manager().has_session(7));
        assert_eq!(
            executor.network_effects(),
            &[PlayerNetworkEffect::WriteResponse {
                connection_id: 7,
                packet: "#HELLO##".to_owned(),
            }]
        );
        assert_eq!(
            executor.connection_logs(),
            &["[7] Connection from 127.0.0.1".to_owned()]
        );

        executor.apply_all(
            &mut server_handler,
            IncomingContext::new(),
            connection_handler.channel_closed(7, "/127.0.0.1:37120"),
        );

        assert!(!server_handler.session_manager().has_session(7));
        assert!(executor
            .session_effects()
            .contains(&SessionLifecycleEffect::RemovePlayer { connection_id: 7 }));
    }

    #[test]
    fn dispatches_request_effects_through_server_message_handler() {
        let connection_handler = ServerConnectionHandler::new(false, true);
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let mut executor = ServerConnectionEffectExecutor::new();
        let effects =
            connection_handler.message_received(9, Some(NettyRequest::from_content("CHAT hello")));

        let context = executor.apply_all(
            &mut server_handler,
            IncomingContext::new().in_room(true),
            effects,
        );

        assert_eq!(
            executor.packet_logs(),
            &["[9] Received: CHAT hello".to_owned()]
        );
        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::Talk {
                    mode: "CHAT".to_owned(),
                    message: "hello".to_owned(),
                }
            ]
        );
    }

    #[test]
    fn applies_exception_effect_as_network_close() {
        let connection_handler = ServerConnectionHandler::new(false, false);
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let mut executor = ServerConnectionEffectExecutor::new();

        executor.apply_all(
            &mut server_handler,
            IncomingContext::new(),
            connection_handler.exception_caught(3),
        );

        assert_eq!(
            executor.network_effects(),
            &[PlayerNetworkEffect::CloseConnection { connection_id: 3 }]
        );
    }
}
