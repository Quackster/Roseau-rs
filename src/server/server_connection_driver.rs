use crate::messages::IncomingContext;
use crate::protocol::DecodeError;
use crate::server::{
    NetworkFrameDecoder, ServerConnectionEffectExecutor, ServerConnectionHandler, ServerHandler,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConnectionDriver {
    connection_id: i32,
    remote_address: String,
    decoder: NetworkFrameDecoder,
    context: IncomingContext,
}

impl ServerConnectionDriver {
    pub fn new(connection_id: i32, remote_address: impl Into<String>) -> Self {
        Self {
            connection_id,
            remote_address: remote_address.into(),
            decoder: NetworkFrameDecoder::new(),
            context: IncomingContext::new(),
        }
    }

    pub fn with_context(mut self, context: IncomingContext) -> Self {
        self.context = context;
        self
    }

    pub fn connection_id(&self) -> i32 {
        self.connection_id
    }

    pub fn remote_address(&self) -> &str {
        &self.remote_address
    }

    pub fn context(&self) -> &IncomingContext {
        &self.context
    }

    pub fn buffered_len(&self) -> usize {
        self.decoder.buffered_len()
    }

    pub fn open(
        &mut self,
        server_handler: &mut ServerHandler,
        connection_handler: &ServerConnectionHandler,
        effect_executor: &mut ServerConnectionEffectExecutor,
    ) {
        self.apply_effects(
            server_handler,
            effect_executor,
            connection_handler.channel_open(self.connection_id, &self.remote_address),
        );
    }

    pub fn read_bytes(
        &mut self,
        bytes: impl AsRef<[u8]>,
        server_handler: &mut ServerHandler,
        connection_handler: &ServerConnectionHandler,
        effect_executor: &mut ServerConnectionEffectExecutor,
    ) -> Result<(), DecodeError> {
        match self.decoder.push_bytes(bytes) {
            Ok(requests) => {
                for request in requests {
                    self.apply_effects(
                        server_handler,
                        effect_executor,
                        connection_handler.message_received(self.connection_id, Some(request)),
                    );
                }
                Ok(())
            }
            Err(error) => {
                self.apply_effects(
                    server_handler,
                    effect_executor,
                    connection_handler.exception_caught(self.connection_id),
                );
                Err(error)
            }
        }
    }

    pub fn close(
        &mut self,
        server_handler: &mut ServerHandler,
        connection_handler: &ServerConnectionHandler,
        effect_executor: &mut ServerConnectionEffectExecutor,
    ) {
        self.apply_effects(
            server_handler,
            effect_executor,
            connection_handler.channel_closed(self.connection_id, &self.remote_address),
        );
    }

    fn apply_effects(
        &mut self,
        server_handler: &mut ServerHandler,
        effect_executor: &mut ServerConnectionEffectExecutor,
        effects: impl IntoIterator<Item = crate::server::ServerConnectionEffect>,
    ) {
        let context = std::mem::take(&mut self.context);
        self.context = effect_executor.apply_all(server_handler, context, effects);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::IncomingCommand;
    use crate::protocol::DecodeError;
    use crate::server::{PlayerNetworkEffect, SessionLifecycleEffect};

    #[test]
    fn opens_and_closes_connection_through_handler_effects() {
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(true, false);
        let mut effect_executor = ServerConnectionEffectExecutor::new();
        let mut driver = ServerConnectionDriver::new(7, "/127.0.0.1:37120");

        driver.open(
            &mut server_handler,
            &connection_handler,
            &mut effect_executor,
        );

        assert!(server_handler.session_manager().has_session(7));
        assert_eq!(
            effect_executor.network_effects(),
            &[PlayerNetworkEffect::WriteResponse {
                connection_id: 7,
                packet: "#HELLO##".to_owned(),
            }]
        );

        driver.close(
            &mut server_handler,
            &connection_handler,
            &mut effect_executor,
        );

        assert!(!server_handler.session_manager().has_session(7));
        assert!(effect_executor
            .session_effects()
            .contains(&SessionLifecycleEffect::RemovePlayer { connection_id: 7 }));
    }

    #[test]
    fn buffers_partial_reads_and_dispatches_complete_requests() {
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(false, true);
        let mut effect_executor = ServerConnectionEffectExecutor::new();
        let mut driver = ServerConnectionDriver::new(9, "/127.0.0.1:37120")
            .with_context(IncomingContext::new().in_room(true));

        driver
            .read_bytes(
                b"0010CHAT",
                &mut server_handler,
                &connection_handler,
                &mut effect_executor,
            )
            .unwrap();

        assert_eq!(driver.buffered_len(), 8);
        assert!(driver.context().commands().is_empty());

        driver
            .read_bytes(
                b" hello",
                &mut server_handler,
                &connection_handler,
                &mut effect_executor,
            )
            .unwrap();

        assert_eq!(
            driver.context().commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::Talk {
                    mode: "CHAT".to_owned(),
                    message: "hello".to_owned(),
                }
            ]
        );
        assert_eq!(
            effect_executor.packet_logs(),
            &["[9] Received: CHAT hello".to_owned()]
        );
    }

    #[test]
    fn closes_connection_on_decode_error() {
        let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
        let connection_handler = ServerConnectionHandler::new(false, false);
        let mut effect_executor = ServerConnectionEffectExecutor::new();
        let mut driver = ServerConnectionDriver::new(11, "/127.0.0.1:37120");

        let error = driver
            .read_bytes(
                b"ABCDCHAT hello",
                &mut server_handler,
                &connection_handler,
                &mut effect_executor,
            )
            .unwrap_err();

        assert_eq!(error, DecodeError::InvalidLength);
        assert_eq!(
            effect_executor.network_effects(),
            &[PlayerNetworkEffect::CloseConnection { connection_id: 11 }]
        );
    }
}
