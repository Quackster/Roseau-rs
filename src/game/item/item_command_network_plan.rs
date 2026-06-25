use crate::game::item::ItemCommandExecution;
use crate::game::player::PlayerManager;
use crate::messages::outgoing::{
    ActiveObjectAdd, ActiveObjectRemove, ActiveObjectUpdate, AddWallItem, RemoveWallItem,
    StuffDataUpdate, UpdateWallItem,
};
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemCommandNetworkPlan;

impl ItemCommandNetworkPlan {
    pub fn plan(
        execution: &ItemCommandExecution,
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        match execution {
            ItemCommandExecution::StuffDataUpdated(item)
            | ItemCommandExecution::RuntimeUpdated(item) => {
                let packet = StuffDataUpdate::new(
                    item.padding(),
                    item.id(),
                    item.definition().data_class(),
                    item.custom_data().unwrap_or_default(),
                )
                .compose()
                .get();
                Self::broadcast(room_player_ids, player_manager, packet)
            }
            ItemCommandExecution::RoomItemPlaced(item) => {
                let packet = if item.definition().behaviour().is_on_floor() {
                    ActiveObjectAdd::new(item.clone()).compose().get()
                } else if item.definition().behaviour().is_on_wall() {
                    AddWallItem::new(item.clone()).compose().get()
                } else {
                    return Vec::new();
                };

                Self::broadcast(room_player_ids, player_manager, packet)
            }
            ItemCommandExecution::RoomItemMoved(item) => {
                let packet = if item.definition().behaviour().is_on_floor() {
                    ActiveObjectUpdate::new(Some(item.clone())).compose().get()
                } else if item.definition().behaviour().is_on_wall() {
                    UpdateWallItem::new(item.clone()).compose().get()
                } else {
                    return Vec::new();
                };

                Self::broadcast(room_player_ids, player_manager, packet)
            }
            ItemCommandExecution::RoomItemDeleted(item)
            | ItemCommandExecution::RoomItemReturned(item) => {
                let packet = if item.definition().behaviour().is_on_floor() {
                    ActiveObjectRemove::new(item.padding(), item.id())
                        .compose()
                        .get()
                } else if item.definition().behaviour().is_on_wall() {
                    RemoveWallItem::new(item.id()).compose().get()
                } else {
                    return Vec::new();
                };

                Self::broadcast(room_player_ids, player_manager, packet)
            }
            ItemCommandExecution::Updated(_)
            | ItemCommandExecution::Deleted { .. }
            | ItemCommandExecution::Ignored => Vec::new(),
        }
    }

    pub fn plan_all(
        executions: &[ItemCommandExecution],
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        executions
            .iter()
            .flat_map(|execution| Self::plan(execution, room_player_ids, player_manager))
            .collect()
    }

    fn broadcast(
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
        packet: String,
    ) -> Vec<PlayerNetworkEffect> {
        room_player_ids
            .iter()
            .filter_map(|user_id| player_manager.get_by_id(*user_id))
            .map(|session| PlayerNetworkEffect::WriteResponse {
                connection_id: session.connection_id(),
                packet: packet.clone(),
            })
            .collect()
    }
}
