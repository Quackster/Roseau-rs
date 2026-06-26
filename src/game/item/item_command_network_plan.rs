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
        Self::packet(execution)
            .map(|packet| Self::broadcast(room_player_ids, player_manager, packet))
            .unwrap_or_default()
    }

    pub fn plan_for_connection_ids(
        execution: &ItemCommandExecution,
        room_connection_ids: &[i32],
    ) -> Vec<PlayerNetworkEffect> {
        Self::packet(execution)
            .map(|packet| Self::broadcast_to_connection_ids(room_connection_ids, packet))
            .unwrap_or_default()
    }

    fn packet(execution: &ItemCommandExecution) -> Option<String> {
        match execution {
            ItemCommandExecution::StuffDataUpdated(item)
            | ItemCommandExecution::RuntimeUpdated(item) => Some(
                StuffDataUpdate::new(
                    item.padding(),
                    item.id(),
                    item.definition().data_class(),
                    item.custom_data().unwrap_or_default(),
                )
                .compose()
                .get(),
            ),
            ItemCommandExecution::RoomItemPlaced(item) => {
                Some(if item.definition().behaviour().is_on_floor() {
                    ActiveObjectAdd::new(item.clone()).compose().get()
                } else if item.definition().behaviour().is_on_wall() {
                    AddWallItem::new(item.clone()).compose().get()
                } else {
                    return None;
                })
            }
            ItemCommandExecution::RoomItemMoved(item) => {
                Some(if item.definition().behaviour().is_on_floor() {
                    ActiveObjectUpdate::new(Some(item.clone())).compose().get()
                } else if item.definition().behaviour().is_on_wall() {
                    UpdateWallItem::new(item.clone()).compose().get()
                } else {
                    return None;
                })
            }
            ItemCommandExecution::RoomItemDeleted(item)
            | ItemCommandExecution::RoomItemReturned(item) => {
                Some(if item.definition().behaviour().is_on_floor() {
                    ActiveObjectRemove::new(item.padding(), item.id())
                        .compose()
                        .get()
                } else if item.definition().behaviour().is_on_wall() {
                    RemoveWallItem::new(item.id()).compose().get()
                } else {
                    return None;
                })
            }
            ItemCommandExecution::Updated(_)
            | ItemCommandExecution::Deleted { .. }
            | ItemCommandExecution::Ignored => None,
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

    pub fn plan_all_for_connection_ids(
        executions: &[ItemCommandExecution],
        room_connection_ids: &[i32],
    ) -> Vec<PlayerNetworkEffect> {
        executions
            .iter()
            .flat_map(|execution| Self::plan_for_connection_ids(execution, room_connection_ids))
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

    fn broadcast_to_connection_ids(
        room_connection_ids: &[i32],
        packet: String,
    ) -> Vec<PlayerNetworkEffect> {
        room_connection_ids
            .iter()
            .map(|connection_id| PlayerNetworkEffect::WriteResponse {
                connection_id: *connection_id,
                packet: packet.clone(),
            })
            .collect()
    }
}

#[cfg(test)]
#[path = "item_command_network_plan_tests.rs"]
mod tests;
