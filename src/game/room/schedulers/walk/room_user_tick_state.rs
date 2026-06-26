use std::collections::HashMap;

use crate::game::room::entity::RoomUserStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomUserTickState {
    entity_id: i32,
    walking: bool,
    needs_update: bool,
    look_reset_time: i64,
    body_rotation: i32,
    head_rotation: i32,
    time_until_next_drink: i64,
    statuses: HashMap<String, RoomUserStatus>,
}

impl RoomUserTickState {
    pub fn new(entity_id: i32) -> Self {
        Self {
            entity_id,
            walking: false,
            needs_update: false,
            look_reset_time: -1,
            body_rotation: 0,
            head_rotation: 0,
            time_until_next_drink: -1,
            statuses: HashMap::new(),
        }
    }

    pub fn walking(mut self, walking: bool) -> Self {
        self.walking = walking;
        self
    }

    pub fn needs_update(mut self, needs_update: bool) -> Self {
        self.needs_update = needs_update;
        self
    }

    pub fn look_reset_time(mut self, look_reset_time: i64) -> Self {
        self.look_reset_time = look_reset_time;
        self
    }

    pub fn rotations(mut self, body_rotation: i32, head_rotation: i32) -> Self {
        self.body_rotation = body_rotation;
        self.head_rotation = head_rotation;
        self
    }

    pub fn time_until_next_drink(mut self, time_until_next_drink: i64) -> Self {
        self.time_until_next_drink = time_until_next_drink;
        self
    }

    pub fn with_status(mut self, status: RoomUserStatus) -> Self {
        self.statuses.insert(status.key().to_owned(), status);
        self
    }

    pub fn entity_id(&self) -> i32 {
        self.entity_id
    }

    pub fn is_walking(&self) -> bool {
        self.walking
    }

    pub fn needs_update_value(&self) -> bool {
        self.needs_update
    }

    pub fn look_reset_time_value(&self) -> i64 {
        self.look_reset_time
    }

    pub fn body_rotation(&self) -> i32 {
        self.body_rotation
    }

    pub fn head_rotation(&self) -> i32 {
        self.head_rotation
    }

    pub fn time_until_next_drink_value(&self) -> i64 {
        self.time_until_next_drink
    }

    pub fn contains_status(&self, key: &str) -> bool {
        self.statuses.contains_key(key)
    }

    pub fn statuses(&self) -> &HashMap<String, RoomUserStatus> {
        &self.statuses
    }
}
