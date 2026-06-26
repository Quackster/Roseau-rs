#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerNetworkEffect {
    WriteResponse { connection_id: i32, packet: String },
    CloseConnection { connection_id: i32 },
}

impl PlayerNetworkEffect {
    pub fn connection_id(&self) -> i32 {
        match self {
            Self::WriteResponse { connection_id, .. } | Self::CloseConnection { connection_id } => {
                *connection_id
            }
        }
    }
}
