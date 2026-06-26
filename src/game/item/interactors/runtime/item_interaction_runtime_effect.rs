use crate::game::item::interactors::ItemInteractionEffect;
use crate::game::room::model::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemInteractionRuntimeEffect {
    SyncPlayerTickets {
        user_id: i32,
        tickets: i32,
    },
    ScheduleEffects {
        user_id: i32,
        delay_ms: u64,
        effects: Vec<ItemInteractionEffect>,
    },
    LoadRoom {
        user_id: i32,
        room_id: i32,
        position: Position,
        rotation: i32,
    },
    LeaveRoom {
        user_id: i32,
        room_id: i32,
    },
}
