#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomEffect {
    StartPublicServer {
        room_name: String,
        port: i32,
    },
    ScheduleWalkTicks,
    ScheduleEventTicks,
    LoadPassiveObjects {
        model_name: String,
        room_id: i32,
    },
    LoadBots {
        room_id: i32,
    },
    RegenerateCollisionMaps,
    RegisterEvent {
        event_name: String,
    },
    SendDoorbell {
        user_id: i32,
        username: String,
    },
    SendOwnerPrivileges {
        user_id: i32,
    },
    SendControllerPrivileges {
        user_id: i32,
    },
    SendNoControllerPrivileges {
        user_id: i32,
    },
    SetRoomUserStatus {
        user_id: i32,
        key: String,
        value: String,
    },
    RemoveRoomUserStatus {
        user_id: i32,
        key: String,
    },
    MarkRoomUserForUpdate {
        user_id: i32,
    },
    LetUserIn {
        user_id: i32,
        room_id: i32,
    },
    LeaveRoom {
        user_id: i32,
        hotel_view: bool,
    },
    KickUser {
        user_id: i32,
    },
    ClearRuntimeData,
    RemoveLoadedRoom {
        room_id: i32,
    },
    SaveRights {
        room_id: i32,
        rights: Vec<i32>,
    },
}
