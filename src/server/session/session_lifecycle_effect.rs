#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionLifecycleEffect {
    CreatePlayerNetwork {
        connection_id: i32,
        server_port: u16,
    },
    AttachPlayer {
        connection_id: i32,
    },
    RegisterPlayer {
        connection_id: i32,
    },
    StoreSession {
        connection_id: i32,
    },
    RemovePlayer {
        connection_id: i32,
    },
    RemoveSession {
        connection_id: i32,
    },
}
