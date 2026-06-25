#[derive(Debug, Clone, PartialEq)]
pub struct RoomTile {
    height: f64,
    override_lock: bool,
    item_ids: Vec<i32>,
    highest_item_id: Option<i32>,
}

impl RoomTile {
    pub fn new() -> Self {
        Self {
            height: 0.0,
            override_lock: false,
            item_ids: Vec::new(),
            highest_item_id: None,
        }
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    pub fn item_ids(&self) -> &[i32] {
        &self.item_ids
    }

    pub fn add_item_id(&mut self, item_id: i32) {
        self.item_ids.push(item_id);
    }

    pub fn highest_item_id(&self) -> Option<i32> {
        self.highest_item_id
    }

    pub fn set_highest_item_id(&mut self, highest_item_id: Option<i32>) {
        self.highest_item_id = highest_item_id;
    }

    pub fn has_override_lock(&self) -> bool {
        self.override_lock
    }

    pub fn set_override_lock(&mut self, override_lock: bool) {
        self.override_lock = override_lock;
    }
}

impl Default for RoomTile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "room_tile_tests.rs"]
mod tests;
