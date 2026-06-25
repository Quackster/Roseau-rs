use crate::game::room::RoomLeaveEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomLeavePlan {
    user_id: i32,
    username: String,
    room_id: i32,
    hotel_view: bool,
    has_private_room_connection: bool,
    has_main_server_player: bool,
    current_item: Option<CurrentRoomItem>,
}

impl RoomLeavePlan {
    pub fn new(user_id: i32, username: impl Into<String>, room_id: i32) -> Self {
        Self {
            user_id,
            username: username.into(),
            room_id,
            hotel_view: false,
            has_private_room_connection: false,
            has_main_server_player: false,
            current_item: None,
        }
    }

    pub fn hotel_view(mut self, hotel_view: bool) -> Self {
        self.hotel_view = hotel_view;
        self
    }

    pub fn private_room_connection(mut self, has_private_room_connection: bool) -> Self {
        self.has_private_room_connection = has_private_room_connection;
        self
    }

    pub fn main_server_player(mut self, has_main_server_player: bool) -> Self {
        self.has_main_server_player = has_main_server_player;
        self
    }

    pub fn current_item(mut self, item_id: i32, sprite: impl Into<String>) -> Self {
        self.current_item = Some(CurrentRoomItem::new(item_id, sprite));
        self
    }

    pub fn effects(&self) -> Vec<RoomLeaveEffect> {
        let mut effects = Vec::new();

        if self.hotel_view && self.has_private_room_connection {
            effects.push(RoomLeaveEffect::ClosePrivateRoomConnection {
                user_id: self.user_id,
            });
        }

        effects.push(RoomLeaveEffect::RemovePlayerEntity {
            user_id: self.user_id,
        });

        if let Some(item) = self.current_item.as_ref() {
            if item.opens_when_leaving() {
                effects.push(RoomLeaveEffect::OpenAndUnlockCurrentItem {
                    item_id: item.item_id,
                });
            }
        }

        effects.push(RoomLeaveEffect::DisposeRoomUser {
            user_id: self.user_id,
        });
        effects.push(RoomLeaveEffect::BroadcastLogout {
            username: self.username.clone(),
        });
        effects.push(RoomLeaveEffect::DisposeRoomIfEmpty {
            room_id: self.room_id,
        });
        effects.push(RoomLeaveEffect::DisposeInventory {
            user_id: self.user_id,
        });

        if self.has_main_server_player {
            effects.push(RoomLeaveEffect::RefreshMainMessengerStatus {
                user_id: self.user_id,
            });
        }

        effects
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CurrentRoomItem {
    item_id: i32,
    sprite: String,
}

impl CurrentRoomItem {
    fn new(item_id: i32, sprite: impl Into<String>) -> Self {
        Self {
            item_id,
            sprite: sprite.into(),
        }
    }

    fn opens_when_leaving(&self) -> bool {
        matches!(self.sprite.as_str(), "poolLift" | "poolBooth")
    }
}

#[cfg(test)]
mod tests {
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
}
