use crate::game::player::PlayerEffect;
use crate::game::room::{RoomManager, RoomSummary};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerEffectRoomManagerExecutor;

impl PlayerEffectRoomManagerExecutor {
    pub fn apply(room_manager: &mut RoomManager, effect: &PlayerEffect) -> Vec<RoomSummary> {
        match effect {
            PlayerEffect::DisposeOwnedRooms { user_id } => {
                let room_ids = room_manager
                    .get_player_rooms(*user_id)
                    .into_iter()
                    .map(|room| room.data().id())
                    .collect::<Vec<_>>();

                room_ids
                    .into_iter()
                    .filter_map(|room_id| room_manager.remove_loaded_room(room_id))
                    .collect()
            }
            PlayerEffect::SendAlert(_)
            | PlayerEffect::UpdateLastLogin { .. }
            | PlayerEffect::CloseConnection { .. }
            | PlayerEffect::CloseUserConnections { .. }
            | PlayerEffect::DisposeInventory { .. }
            | PlayerEffect::LeaveCurrentRoom { .. }
            | PlayerEffect::Messenger(_) => Vec::new(),
        }
    }

    pub fn apply_all(room_manager: &mut RoomManager, effects: &[PlayerEffect]) -> Vec<RoomSummary> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(room_manager, effect))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::room::settings::RoomType;
    use crate::game::room::RoomData;
    use crate::messages::outgoing::SystemBroadcast;

    fn room(id: i32, owner_id: i32, hidden: bool) -> RoomSummary {
        RoomSummary::new(RoomData::new(
            id,
            hidden,
            RoomType::Private,
            owner_id,
            "owner",
            format!("room{id}"),
            0,
            "",
            25,
            "desc",
            "model",
            "class",
            "wall",
            "floor",
            false,
            true,
        ))
    }

    #[test]
    fn removes_visible_loaded_rooms_owned_by_player() {
        let mut manager = RoomManager::new();
        manager.add(room(11, 7, false));
        manager.add(room(12, 7, true));
        manager.add(room(13, 8, false));

        let removed = PlayerEffectRoomManagerExecutor::apply(
            &mut manager,
            &PlayerEffect::DisposeOwnedRooms { user_id: 7 },
        );

        assert_eq!(
            removed
                .iter()
                .map(|room| room.data().id())
                .collect::<Vec<_>>(),
            vec![11]
        );
        assert!(manager.get_room_by_id(11).is_none());
        assert!(manager.get_room_by_id(12).is_some());
        assert!(manager.get_room_by_id(13).is_some());
    }

    #[test]
    fn ignores_non_room_manager_player_effects() {
        let mut manager = RoomManager::new();
        manager.add(room(11, 7, false));

        let removed = PlayerEffectRoomManagerExecutor::apply_all(
            &mut manager,
            &[
                PlayerEffect::SendAlert(SystemBroadcast::new("hello")),
                PlayerEffect::DisposeInventory { user_id: 7 },
                PlayerEffect::LeaveCurrentRoom { connection_id: 99 },
            ],
        );

        assert!(removed.is_empty());
        assert!(manager.get_room_by_id(11).is_some());
    }
}
