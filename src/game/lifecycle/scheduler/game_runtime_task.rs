#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameRuntimeTask {
    GameTick,
    RoomWalkTick { room_id: i32 },
    RoomEventTick { room_id: i32 },
    BotResponse { entity_id: i32 },
    TeleporterTransfer { user_id: i32, item_id: i32 },
}
