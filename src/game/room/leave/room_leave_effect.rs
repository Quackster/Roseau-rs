#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomLeaveEffect {
    ClosePrivateRoomConnection { user_id: i32 },
    RemovePlayerEntity { user_id: i32 },
    OpenAndUnlockCurrentItem { item_id: i32 },
    DisposeRoomUser { user_id: i32 },
    BroadcastLogout { username: String },
    DisposeRoomIfEmpty { room_id: i32 },
    DisposeInventory { user_id: i32 },
    RefreshMainMessengerStatus { user_id: i32 },
}
