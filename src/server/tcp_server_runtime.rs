use crate::server::{
    ServerConnectionHandler, ServerHandler, ServerSocketBinder, TcpConnectionAcceptor,
    TcpConnectionRuntime, TcpServerAcceptOutcome, TcpServerStepOutcome, TcpServerTickOutcome,
};

pub struct TcpServerRuntime {
    server_handler: ServerHandler,
    connection_handler: ServerConnectionHandler,
    acceptor: TcpConnectionAcceptor,
    pub(super) connections: Vec<TcpConnectionRuntime>,
}

impl TcpServerRuntime {
    pub fn new(
        server_handler: ServerHandler,
        connection_handler: ServerConnectionHandler,
        acceptor: TcpConnectionAcceptor,
    ) -> Self {
        Self {
            server_handler,
            connection_handler,
            acceptor,
            connections: Vec::new(),
        }
    }

    pub fn with_first_connection_id(
        server_handler: ServerHandler,
        connection_handler: ServerConnectionHandler,
        first_connection_id: i32,
    ) -> Self {
        Self::new(
            server_handler,
            connection_handler,
            TcpConnectionAcceptor::new(first_connection_id),
        )
    }

    pub fn server_handler(&self) -> &ServerHandler {
        &self.server_handler
    }

    pub fn acceptor(&self) -> &TcpConnectionAcceptor {
        &self.acceptor
    }

    pub fn connections(&self) -> &[TcpConnectionRuntime] {
        &self.connections
    }

    pub fn connection(&self, index: usize) -> Option<&TcpConnectionRuntime> {
        self.connections.get(index)
    }

    pub fn accept_and_open_one<B: ServerSocketBinder>(
        &mut self,
        binder: &B,
        listener_index: usize,
    ) -> Result<i32, String> {
        let mut runtime = self.acceptor.accept_one(binder, listener_index)?;
        let connection_id = runtime.connection_id();
        runtime.set_nonblocking(true)?;
        runtime.open(&mut self.server_handler, &self.connection_handler);
        self.connections.push(runtime);

        Ok(connection_id)
    }

    pub fn read_connection(
        &mut self,
        index: usize,
        max_bytes: usize,
    ) -> Result<Option<usize>, String> {
        let runtime = self
            .connections
            .get_mut(index)
            .ok_or_else(|| format!("connection {index} is not active"))?;

        runtime.read_from_network_nonblocking(
            max_bytes,
            &mut self.server_handler,
            &self.connection_handler,
        )
    }

    pub fn read_active_connections(&mut self, max_bytes: usize) -> Vec<TcpServerStepOutcome> {
        let mut outcomes = Vec::new();

        for index in 0..self.connections.len() {
            let connection_id = self.connections[index].connection_id();

            match self.read_connection(index, max_bytes) {
                Ok(None) => outcomes.push(TcpServerStepOutcome::Idle { connection_id }),
                Ok(Some(0)) => outcomes.push(TcpServerStepOutcome::Closed { connection_id }),
                Ok(Some(bytes_read)) => outcomes.push(TcpServerStepOutcome::Read {
                    connection_id,
                    bytes_read,
                }),
                Err(message) => outcomes.push(TcpServerStepOutcome::Error {
                    connection_id,
                    message,
                }),
            }
        }

        outcomes
    }

    pub fn close_connection(&mut self, index: usize) -> Result<i32, String> {
        let runtime = self
            .connections
            .get_mut(index)
            .ok_or_else(|| format!("connection {index} is not active"))?;
        let connection_id = runtime.connection_id();

        runtime.close(&mut self.server_handler, &self.connection_handler);

        Ok(connection_id)
    }

    pub fn remove_connection(&mut self, index: usize) -> Result<i32, String> {
        if index >= self.connections.len() {
            return Err(format!("connection {index} is not active"));
        }

        Ok(self.connections.remove(index).connection_id())
    }

    pub fn remove_closed_connections(&mut self, outcomes: &[TcpServerStepOutcome]) -> Vec<i32> {
        let closed_connection_ids: Vec<i32> = outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                TcpServerStepOutcome::Closed { connection_id } => Some(*connection_id),
                TcpServerStepOutcome::Idle { .. }
                | TcpServerStepOutcome::Read { .. }
                | TcpServerStepOutcome::Error { .. } => None,
            })
            .collect();

        let mut removed_connection_ids = Vec::new();
        self.connections.retain(|connection| {
            let connection_id = connection.connection_id();
            let keep = !closed_connection_ids.contains(&connection_id);

            if !keep {
                removed_connection_ids.push(connection_id);
            }

            keep
        });

        removed_connection_ids
    }

    pub fn step<B: ServerSocketBinder>(
        &mut self,
        binder: &B,
        listener_index: usize,
        accept_connection: bool,
        max_bytes: usize,
    ) -> TcpServerTickOutcome {
        let read_outcomes = self.read_active_connections(max_bytes);
        let removed_connection_ids = self.remove_closed_connections(&read_outcomes);
        let accept_outcome = if accept_connection {
            match self.acceptor.accept_one_nonblocking(binder, listener_index) {
                Ok(Some(mut runtime)) => {
                    let connection_id = runtime.connection_id();
                    if let Err(error) = runtime.set_nonblocking(true) {
                        TcpServerAcceptOutcome::Error { message: error }
                    } else {
                        runtime.open(&mut self.server_handler, &self.connection_handler);
                        self.connections.push(runtime);
                        TcpServerAcceptOutcome::Accepted { connection_id }
                    }
                }
                Ok(None) => TcpServerAcceptOutcome::Idle,
                Err(error) => TcpServerAcceptOutcome::Error { message: error },
            }
        } else {
            TcpServerAcceptOutcome::Skipped
        };

        TcpServerTickOutcome::new(accept_outcome, read_outcomes, removed_connection_ids)
    }

    pub fn step_all_listeners<B: ServerSocketBinder>(
        &mut self,
        binder: &B,
        max_bytes: usize,
    ) -> TcpServerTickOutcome {
        let read_outcomes = self.read_active_connections(max_bytes);
        let removed_connection_ids = self.remove_closed_connections(&read_outcomes);
        let mut accept_outcome = TcpServerAcceptOutcome::Idle;

        for listener_index in 0..self.server_handler.ports().len() {
            match self.acceptor.accept_one_nonblocking(binder, listener_index) {
                Ok(Some(mut runtime)) => {
                    let connection_id = runtime.connection_id();
                    if let Err(error) = runtime.set_nonblocking(true) {
                        if matches!(accept_outcome, TcpServerAcceptOutcome::Idle) {
                            accept_outcome = TcpServerAcceptOutcome::Error { message: error };
                        }
                    } else {
                        runtime.open(&mut self.server_handler, &self.connection_handler);
                        self.connections.push(runtime);
                        if matches!(accept_outcome, TcpServerAcceptOutcome::Idle) {
                            accept_outcome = TcpServerAcceptOutcome::Accepted { connection_id };
                        }
                    }
                }
                Ok(None) => {}
                Err(error) => {
                    if matches!(accept_outcome, TcpServerAcceptOutcome::Idle) {
                        accept_outcome = TcpServerAcceptOutcome::Error { message: error };
                    }
                }
            }
        }

        TcpServerTickOutcome::new(accept_outcome, read_outcomes, removed_connection_ids)
    }
}
