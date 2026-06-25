use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct CommandContext {
    room_user: Option<RoomUserCommandState>,
}

impl CommandContext {
    pub fn new() -> Self {
        Self { room_user: None }
    }

    pub fn with_room_user(room_user: RoomUserCommandState) -> Self {
        Self {
            room_user: Some(room_user),
        }
    }

    pub fn room_user(&self) -> Option<&RoomUserCommandState> {
        self.room_user.as_ref()
    }
}

impl Default for CommandContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RoomUserCommandState {
    in_room: bool,
    walking: bool,
    statuses: HashSet<String>,
    rotation: i32,
    tile_height: f64,
}

impl RoomUserCommandState {
    pub fn new(in_room: bool, walking: bool, rotation: i32, tile_height: f64) -> Self {
        Self {
            in_room,
            walking,
            statuses: HashSet::new(),
            rotation,
            tile_height,
        }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.statuses.insert(status.into());
        self
    }

    pub fn is_in_room(&self) -> bool {
        self.in_room
    }

    pub fn is_walking(&self) -> bool {
        self.walking
    }

    pub fn contains_status(&self, status: &str) -> bool {
        self.statuses.contains(status)
    }

    pub fn rotation(&self) -> i32 {
        self.rotation
    }

    pub fn tile_height(&self) -> f64 {
        self.tile_height
    }
}
