use crate::messages::{IncomingContext, MessageHandler};
use crate::protocol::ClientMessage;
use crate::server::SessionManager;

pub struct ServerHandler {
    ports: Vec<u16>,
    ip_address: String,
    extra_data: Option<String>,
    message_handler: MessageHandler,
    session_manager: SessionManager,
}

impl ServerHandler {
    pub fn new(ports: impl Into<Vec<u16>>, ip_address: impl Into<String>) -> Self {
        Self {
            ports: ports.into(),
            ip_address: ip_address.into(),
            extra_data: None,
            message_handler: MessageHandler::new(),
            session_manager: SessionManager::new(),
        }
    }

    pub fn ports(&self) -> &[u16] {
        &self.ports
    }

    pub fn ip_address(&self) -> &str {
        &self.ip_address
    }

    pub fn set_ip_address(&mut self, ip_address: impl Into<String>) {
        self.ip_address = ip_address.into();
    }

    pub fn extra_data(&self) -> Option<&str> {
        self.extra_data.as_deref()
    }

    pub fn set_extra_data(&mut self, extra_data: impl Into<String>) {
        self.extra_data = Some(extra_data.into());
    }

    pub fn clear_extra_data(&mut self) {
        self.extra_data = None;
    }

    pub fn message_handler(&self) -> &MessageHandler {
        &self.message_handler
    }

    pub fn dispatch_request(
        &self,
        context: IncomingContext,
        message: &dyn ClientMessage,
    ) -> IncomingContext {
        self.message_handler.handle_request(context, message)
    }

    pub fn session_manager(&self) -> &SessionManager {
        &self.session_manager
    }

    pub fn session_manager_mut(&mut self) -> &mut SessionManager {
        &mut self.session_manager
    }

    pub fn open_connection(&mut self, connection_id: i32) {
        self.session_manager.add_session(connection_id);
    }

    pub fn close_connection(&mut self, connection_id: i32) {
        self.session_manager.remove_session(connection_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::IncomingCommand;
    use crate::protocol::NettyRequest;

    #[test]
    fn keeps_server_state_messages_and_sessions() {
        let mut handler = ServerHandler::new(vec![30001, 30002], "127.0.0.1");

        handler.set_ip_address("0.0.0.0");
        handler.set_extra_data("hotel");
        handler.open_connection(5);

        assert_eq!(handler.ports(), &[30001, 30002]);
        assert_eq!(handler.ip_address(), "0.0.0.0");
        assert_eq!(handler.extra_data(), Some("hotel"));
        assert!(handler.message_handler().contains_header("VERSIONCHECK"));
        assert!(handler.session_manager().has_session(5));

        handler.close_connection(5);
        handler.clear_extra_data();

        assert!(!handler.session_manager().has_session(5));
        assert_eq!(handler.extra_data(), None);
    }

    #[test]
    fn dispatches_requests_through_owned_message_handler() {
        let handler = ServerHandler::new(vec![30001], "127.0.0.1");
        let context = handler.dispatch_request(
            IncomingContext::new().in_room(true),
            &NettyRequest::from_content("CHAT hello"),
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
}
