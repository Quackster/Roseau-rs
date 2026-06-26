use crate::messages::IncomingCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingIncomingCommandBatch {
    connection_id: i32,
    server_port: i32,
    commands: Vec<IncomingCommand>,
}

impl PendingIncomingCommandBatch {
    pub fn new(
        connection_id: i32,
        server_port: i32,
        commands: impl Into<Vec<IncomingCommand>>,
    ) -> Self {
        Self {
            connection_id,
            server_port,
            commands: commands.into(),
        }
    }

    pub fn connection_id(&self) -> i32 {
        self.connection_id
    }

    pub fn server_port(&self) -> i32 {
        self.server_port
    }

    pub fn commands(&self) -> &[IncomingCommand] {
        &self.commands
    }
}
