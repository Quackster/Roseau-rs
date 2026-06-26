use crate::messages::IncomingContext;
use crate::protocol::DecodeError;
use crate::server::{
    secret_decode, NetworkFrameDecoder, ServerConnectionEffectExecutor, ServerConnectionHandler,
    ServerHandler,
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

    pub fn context_mut(&mut self) -> &mut IncomingContext {
        &mut self.context
    }

    pub fn buffered_len(&self) -> usize {
        self.decoder.buffered_len()
    }

    pub fn rc4_enabled(&self) -> bool {
        self.decoder.rc4_enabled()
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
        server_port: i32,
        server_handler: &mut ServerHandler,
        connection_handler: &ServerConnectionHandler,
        effect_executor: &mut ServerConnectionEffectExecutor,
    ) -> Result<(), DecodeError> {
        match self.decoder.push_bytes(bytes) {
            Ok(requests) => {
                for request in requests {
                    let enable_rc4 = request.header() == "VERSIONCHECK";
                    self.apply_effects(
                        server_handler,
                        effect_executor,
                        connection_handler.message_received(
                            self.connection_id,
                            server_port,
                            Some(request),
                        ),
                    );
                    if enable_rc4 {
                        self.decoder.enable_rc4(secret_decode(
                            crate::messages::incoming::auth_session::version_check::V1_SECRET_KEY,
                        ));
                    }
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
#[path = "server_connection_driver_tests.rs"]
mod tests;
