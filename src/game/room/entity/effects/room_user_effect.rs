use crate::game::room::model::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum RoomUserEffect {
    Chat {
        header: String,
        username: String,
        message: String,
    },
    Whisper {
        username: String,
        target_username: Option<String>,
        message: String,
    },
    DelayedChat {
        username: String,
        message: String,
        delay_ms: i32,
    },
    SendStatus {
        entity_id: i32,
    },
    SendUsers {
        entity_id: i32,
    },
    ShowProgram(Vec<String>),
    NotEnoughTickets,
    Kick,
    TransferRoom {
        room_id: i32,
        door_position: Option<Position>,
    },
    TriggerCurrentItem {
        item_id: Option<i32>,
    },
    WalkStarted {
        entity_id: i32,
        x: i32,
        y: i32,
    },
}
