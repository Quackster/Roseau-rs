#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    connection_id: i32,
    player_id: Option<i32>,
}

impl Session {
    pub fn new(connection_id: i32) -> Self {
        Self {
            connection_id,
            player_id: None,
        }
    }

    pub fn with_player_id(mut self, player_id: i32) -> Self {
        self.player_id = Some(player_id);
        self
    }

    pub fn set_player_id(&mut self, player_id: i32) {
        self.player_id = Some(player_id);
    }

    pub fn connection_id(&self) -> i32 {
        self.connection_id
    }

    pub fn player_id(&self) -> Option<i32> {
        self.player_id
    }
}
