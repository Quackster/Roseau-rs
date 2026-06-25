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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::item::{Item, ItemDefinition};
    use crate::game::player::{PlayerDetails, PlayerSession};

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    fn manager() -> PlayerManager {
        let mut manager = PlayerManager::new(Vec::new());
        manager.insert(PlayerSession::new(70, 30000, details(7, "alice")));
        manager.insert(PlayerSession::new(80, 30000, details(8, "bob")));
        manager
    }

    fn item_with(id: i32, flags: &str, sprite: &str, data_class: &str, custom_data: &str) -> Item {
        Item::new(
            id,
            1,
            7,
            "1",
            2,
            0.0,
            0,
            ItemDefinition::new(5, sprite, "", 1, 1, 1.0, flags, "", "", data_class),
            "",
            Some(custom_data.to_owned()),
        )
        .unwrap()
    }

    fn item(custom_data: &str) -> Item {
        item_with(42, "SIF", "switch", "SWITCHON", custom_data)
    }

    #[test]
    fn broadcasts_stuff_data_updates_to_room_players() {
        let manager = manager();

        let effects = ItemCommandNetworkPlan::plan(
            &ItemCommandExecution::StuffDataUpdated(item("OFF")),
            &[7, 8],
            &manager,
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#STUFFDATAUPDATE\r00000042//SWITCHON/OFF##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 80,
                    packet: "#STUFFDATAUPDATE\r00000042//SWITCHON/OFF##".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn skips_non_stuff_data_item_command_results() {
        let manager = manager();

        assert!(ItemCommandNetworkPlan::plan(
            &ItemCommandExecution::Updated(item("ON")),
            &[7],
            &manager,
        )
        .is_empty());
        assert!(
            ItemCommandNetworkPlan::plan(&ItemCommandExecution::Ignored, &[7], &manager).is_empty()
        );
    }

    #[test]
    fn broadcasts_room_item_removals_like_java_mapping_remove() {
        let manager = manager();

        let effects = ItemCommandNetworkPlan::plan_all(
            &[
                ItemCommandExecution::RoomItemDeleted(item_with(42, "SIF", "chair", "", "")),
                ItemCommandExecution::RoomItemDeleted(item_with(55, "SIW", "poster", "", "")),
                ItemCommandExecution::RoomItemReturned(item_with(66, "SIF", "table", "", "")),
            ],
            &[7],
            &manager,
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#ACTIVEOBJECT_REMOVE\r0000042##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#REMOVEITEM\r55##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#ACTIVEOBJECT_REMOVE\r0000066##".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn broadcasts_room_item_placements_like_java_mapping_add() {
        let manager = manager();

        let effects = ItemCommandNetworkPlan::plan_all(
            &[
                ItemCommandExecution::RoomItemPlaced(item_with(42, "SIF", "chair", "", "")),
                ItemCommandExecution::RoomItemPlaced(item_with(55, "SIW", "poster", "", "")),
            ],
            &[7],
            &manager,
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#ACTIVEOBJECT_ADD\r0000042,chair 1 2 1 1 0 0 //##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#ADDITEM\r55;poster;Alex;1\r##".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn broadcasts_room_item_moves_like_java_mapping_position_update() {
        let manager = manager();

        let effects = ItemCommandNetworkPlan::plan_all(
            &[
                ItemCommandExecution::RoomItemMoved(item_with(42, "SIF", "chair", "", "")),
                ItemCommandExecution::RoomItemMoved(item_with(55, "SIW", "poster", "", "")),
            ],
            &[7],
            &manager,
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#ACTIVEOBJECT_UPDATE\r0000042,chair 1 2 1 1 0 0 //##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#UPDATEITEM\r55;poster;Alex;1\r##".to_owned(),
                },
            ]
        );
    }
}
