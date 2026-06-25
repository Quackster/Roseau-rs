use super::*;

#[test]
fn plans_standard_room_leave_effects() {
    let effects = RoomLeavePlan::new(7, "alice", 12)
        .main_server_player(true)
        .effects();

    assert_eq!(
        effects,
        vec![
            RoomLeaveEffect::RemovePlayerEntity { user_id: 7 },
            RoomLeaveEffect::DisposeRoomUser { user_id: 7 },
            RoomLeaveEffect::BroadcastLogout {
                username: "alice".to_owned(),
            },
            RoomLeaveEffect::DisposeRoomIfEmpty { room_id: 12 },
            RoomLeaveEffect::DisposeInventory { user_id: 7 },
            RoomLeaveEffect::RefreshMainMessengerStatus { user_id: 7 },
        ]
    );
}

#[test]
fn closes_private_connection_and_opens_pool_item_when_leaving_to_hotel_view() {
    let effects = RoomLeavePlan::new(7, "alice", 12)
        .hotel_view(true)
        .private_room_connection(true)
        .current_item(99, "poolLift")
        .effects();

    assert_eq!(
        effects,
        vec![
            RoomLeaveEffect::ClosePrivateRoomConnection { user_id: 7 },
            RoomLeaveEffect::RemovePlayerEntity { user_id: 7 },
            RoomLeaveEffect::OpenAndUnlockCurrentItem { item_id: 99 },
            RoomLeaveEffect::DisposeRoomUser { user_id: 7 },
            RoomLeaveEffect::BroadcastLogout {
                username: "alice".to_owned(),
            },
            RoomLeaveEffect::DisposeRoomIfEmpty { room_id: 12 },
            RoomLeaveEffect::DisposeInventory { user_id: 7 },
        ]
    );
}

#[test]
fn ignores_non_pool_lift_or_booth_current_items() {
    let effects = RoomLeavePlan::new(7, "alice", 12)
        .current_item(99, "chair")
        .effects();

    assert!(!effects.contains(&RoomLeaveEffect::OpenAndUnlockCurrentItem { item_id: 99 }));
}
