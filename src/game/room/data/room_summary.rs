use crate::game::room::RoomData;
use crate::game::room::schedulers::RoomScheduler;

#[derive(Debug, Clone, PartialEq)]
pub struct RoomSummary {
    data: RoomData,
    order_id: i32,
    player_count: usize,
    scheduler: Option<RoomScheduler>,
}

impl RoomSummary {
    pub fn new(data: RoomData) -> Self {
        Self {
            data,
            order_id: -1,
            player_count: 0,
            scheduler: None,
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

    pub fn scheduler(&self) -> Option<&RoomScheduler> {
        self.scheduler.as_ref()
    }

    pub fn scheduler_mut(&mut self) -> Option<&mut RoomScheduler> {
        self.scheduler.as_mut()
    }

    pub fn set_scheduler(&mut self, scheduler: Option<RoomScheduler>) {
        self.scheduler = scheduler;
    }
}
