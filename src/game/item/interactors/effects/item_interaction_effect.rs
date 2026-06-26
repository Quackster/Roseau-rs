use crate::game::room::model::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemInteractionEffect {
    RemoveStatus {
        status: String,
    },
    SetStatus {
        status: String,
        value: String,
        persistent: bool,
        ticks: i32,
    },
    SetBodyRotation {
        rotation: i32,
    },
    SetPosition {
        position: Position,
    },
    SetCanWalk {
        can_walk: bool,
    },
    SetWalking {
        walking: bool,
    },
    ClearNextStep,
    ForceStopWalking,
    MarkNeedsUpdate,
    SetGoal {
        position: Position,
    },
    BuildPathToGoal,
    TriggerCurrentItem,
    WalkTo {
        x: i32,
        y: i32,
    },
    ShowProgram {
        item_id: i32,
        program: String,
    },
    LockTiles {
        item_id: i32,
    },
    UnlockTiles {
        item_id: i32,
    },
    OpenPoolChangeBooth,
    SendJumpingPlaceOk,
    SendJumpData {
        username: String,
        data: String,
    },
    DecrementTickets {
        amount: i32,
    },
    SendTickets,
    SavePlayer,
    SendDoorOut {
        item_id: i32,
    },
    SendDoorIn {
        item_id: i32,
    },
    LoadRoom {
        room_id: i32,
        position: Position,
        rotation: i32,
    },
    LeaveRoom {
        room_id: i32,
    },
    SetItemCustomData {
        item_id: i32,
        custom_data: String,
    },
    UpdateItemStatus {
        item_id: i32,
    },
    Schedule {
        delay_ms: u64,
        effects: Vec<ItemInteractionEffect>,
    },
}
