use crate::protocol::NettyRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerConnectionEffect {
    SendHello {
        connection_id: i32,
    },
    AddSession {
        connection_id: i32,
    },
    RemoveSession {
        connection_id: i32,
    },
    DisposePlayer {
        connection_id: i32,
    },
    LogConnection {
        line: String,
    },
    LogPacket {
        line: String,
    },
    DispatchRequest {
        connection_id: i32,
        server_port: i32,
        request: NettyRequest,
    },
    CloseConnection {
        connection_id: i32,
    },
}
