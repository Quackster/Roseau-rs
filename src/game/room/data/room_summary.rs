use crate::game::room::RoomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomSummary {
    data: RoomData,
    order_id: i32,
    player_count: usize,
}

impl RoomSummary {
    pub fn new(data: RoomData) -> Self {
        Self {
            data,
            order_id: -1,
            player_count: 0,
        }
    }

    pub fn data(&self) -> &RoomData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut RoomData {
        &mut self.data
    }

    pub fn order_id(&self) -> i32 {
        self.order_id
    }

    pub fn set_order_id(&mut self, order_id: i32) {
        self.order_id = order_id;
    }

    pub fn player_count(&self) -> usize {
        self.player_count
    }

    pub fn set_player_count(&mut self, player_count: usize) {
        self.player_count = player_count;
    }
}
