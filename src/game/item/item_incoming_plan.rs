use crate::dao::{DaoError, ItemDao};
use crate::game::item::{ItemCommandExecution, ItemCommandExecutor};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemIncomingPlan;

impl ItemIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        item_dao: &dyn ItemDao,
        room_id: i32,
        room_owner_id: i32,
        inventory_owner_id: i32,
        has_rights: bool,
        has_owner_rights: bool,
        all_super_user: bool,
        move_dir_rotation: Option<i32>,
    ) -> Result<Vec<ItemCommandExecution>, DaoError> {
        let execution = match effect {
            IncomingExecutionEffect::SetItemData { item_id, data } => {
                ItemCommandExecutor::set_item_data(item_dao, *item_id, data, has_rights)?
            }
            IncomingExecutionEffect::SetStuffData {
                item_id,
                data_class,
                custom_data,
            } => ItemCommandExecutor::set_stuff_data(item_dao, *item_id, data_class, custom_data)?,
            IncomingExecutionEffect::UseStripItem { item_id } => {
                ItemCommandExecutor::use_strip_item(item_dao, *item_id)?
            }
            IncomingExecutionEffect::RemoveItem { item_id } => {
                ItemCommandExecutor::remove_item(item_dao, *item_id, has_owner_rights)?
            }
            IncomingExecutionEffect::ReturnItemToInventory { item_id } => {
                ItemCommandExecutor::return_item_to_inventory(
                    item_dao,
                    *item_id,
                    inventory_owner_id,
                    has_owner_rights,
                )?
            }
            IncomingExecutionEffect::PlaceWallItemFromInventory {
                item_id,
                wall_position,
            } => ItemCommandExecutor::place_wall_item(
                item_dao,
                *item_id,
                room_id,
                room_owner_id,
                wall_position,
                has_rights,
                all_super_user,
            )?,
            IncomingExecutionEffect::PlaceFloorItemFromInventory { item_id, x, y, .. } => {
                ItemCommandExecutor::place_floor_item(
                    item_dao,
                    *item_id,
                    room_id,
                    room_owner_id,
                    *x,
                    *y,
                    has_rights,
                    all_super_user,
                )?
            }
            IncomingExecutionEffect::MoveStuff {
                item_id,
                x,
                y,
                rotation,
            } => ItemCommandExecutor::move_stuff(
                item_dao,
                *item_id,
                *x,
                *y,
                *rotation,
                move_dir_rotation,
                has_rights,
                all_super_user,
            )?,
            _ => return Ok(Vec::new()),
        };

        Ok(vec![execution])
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        item_dao: &dyn ItemDao,
        room_id: i32,
        room_owner_id: i32,
        inventory_owner_id: i32,
        has_rights: bool,
        has_owner_rights: bool,
        all_super_user: bool,
        move_dir_rotation: Option<i32>,
    ) -> Result<Vec<ItemCommandExecution>, DaoError> {
        let mut executions = Vec::new();

        for effect in effects {
            executions.extend(Self::plan(
                effect,
                item_dao,
                room_id,
                room_owner_id,
                inventory_owner_id,
                has_rights,
                has_owner_rights,
                all_super_user,
                move_dir_rotation,
            )?);
        }

        Ok(executions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::InMemoryItemDao;
    use crate::dao::ItemDao;
    use crate::game::item::{Item, ItemDefinition};

    fn definition(id: i32, flags: &str, data_class: &str) -> ItemDefinition {
        ItemDefinition::new(
            id, "sprite", "red", 1, 1, 1.0, flags, "Name", "", data_class,
        )
    }

    fn item(id: i32, definition: ItemDefinition, custom_data: &str) -> Item {
        Item::new(
            id,
            0,
            7,
            "1",
            2,
            0.0,
            0,
            definition,
            "",
            Some(custom_data.to_owned()),
        )
        .unwrap()
    }

    #[test]
    fn plans_data_update_strip_use_and_room_item_removal() {
        let dao = InMemoryItemDao::new();
        dao.insert_item(item(1, definition(5, "SIF", ""), "old"));
        dao.insert_item(item(2, definition(6, "SIF", "SWITCHON"), "ON"));
        dao.insert_item(item(3, definition(7, "SIFJ", ""), "2"));
        dao.insert_item(item(4, definition(8, "SIF", ""), ""));

        let executions = ItemIncomingPlan::plan_all(
            &[
                IncomingExecutionEffect::SetItemData {
                    item_id: 1,
                    data: "old value".to_owned(),
                },
                IncomingExecutionEffect::SetStuffData {
                    item_id: 2,
                    data_class: "SWITCHON".to_owned(),
                    custom_data: "BROKEN".to_owned(),
                },
                IncomingExecutionEffect::UseStripItem { item_id: 3 },
                IncomingExecutionEffect::RemoveItem { item_id: 4 },
            ],
            &dao,
            42,
            9,
            9,
            true,
            true,
            false,
            None,
        )
        .unwrap();

        assert!(matches!(executions[0], ItemCommandExecution::Updated(_)));
        assert!(matches!(
            executions[1],
            ItemCommandExecution::StuffDataUpdated(_)
        ));
        assert!(matches!(executions[2], ItemCommandExecution::Updated(_)));
        assert!(matches!(
            executions[3],
            ItemCommandExecution::RoomItemDeleted(_)
        ));
        assert_eq!(
            dao.item(1).unwrap().unwrap().custom_data(),
            Some("old value")
        );
        assert_eq!(dao.item(2).unwrap().unwrap().custom_data(), Some("OFF"));
        assert_eq!(dao.item(3).unwrap().unwrap().custom_data(), Some("1"));
        assert!(dao.item(4).unwrap().is_none());
    }

    #[test]
    fn plans_inventory_placement_and_return_with_room_context() {
        let dao = InMemoryItemDao::new();
        dao.insert_item(item(10, definition(5, "SIW", ""), "paper"));
        dao.insert_item(item(11, definition(6, "SIF", ""), ""));
        dao.insert_item(item(12, definition(7, "SIF", ""), ""));

        let executions = ItemIncomingPlan::plan_all(
            &[
                IncomingExecutionEffect::PlaceWallItemFromInventory {
                    item_id: 10,
                    wall_position: ":w=1,1/paper".to_owned(),
                },
                IncomingExecutionEffect::PlaceFloorItemFromInventory {
                    item_id: 11,
                    x: 3,
                    y: 4,
                    rotation: 2,
                },
                IncomingExecutionEffect::ReturnItemToInventory { item_id: 12 },
            ],
            &dao,
            42,
            9,
            99,
            true,
            true,
            false,
            None,
        )
        .unwrap();

        assert!(matches!(
            executions[0],
            ItemCommandExecution::RoomItemPlaced(_)
        ));
        assert!(matches!(
            executions[1],
            ItemCommandExecution::RoomItemPlaced(_)
        ));
        assert!(matches!(
            executions[2],
            ItemCommandExecution::RoomItemReturned(_)
        ));
        assert_eq!(dao.item(10).unwrap().unwrap().room_id(), 42);
        assert_eq!(
            dao.item(10).unwrap().unwrap().wall_position(),
            Some(":w=1,1")
        );
        assert_eq!(dao.item(11).unwrap().unwrap().position().x(), 3);
        assert_eq!(dao.item(12).unwrap().unwrap().owner_id(), 99);
    }

    #[test]
    fn plans_move_stuff_with_sampled_dir_rotation() {
        let dao = InMemoryItemDao::new();
        dao.insert_item(item(20, definition(5, "SIF", "DIR"), "old"));

        let executions = ItemIncomingPlan::plan(
            &IncomingExecutionEffect::MoveStuff {
                item_id: 20,
                x: 5,
                y: 6,
                rotation: Some(2),
            },
            &dao,
            42,
            9,
            9,
            true,
            false,
            false,
            Some(6),
        )
        .unwrap();

        let ItemCommandExecution::RoomItemMoved(item) = &executions[0] else {
            panic!("expected movement");
        };
        assert_eq!(item.position().x(), 5);
        assert_eq!(item.position().rotation(), 6);
        assert_eq!(item.custom_data(), Some("6"));
    }

    #[test]
    fn ignores_unrelated_effects_and_missing_rights() {
        let dao = InMemoryItemDao::new();
        dao.insert_item(item(30, definition(5, "SIF", ""), ""));

        assert!(ItemIncomingPlan::plan(
            &IncomingExecutionEffect::GoAway,
            &dao,
            42,
            9,
            9,
            false,
            false,
            false,
            None,
        )
        .unwrap()
        .is_empty());
        assert_eq!(
            ItemIncomingPlan::plan(
                &IncomingExecutionEffect::RemoveItem { item_id: 30 },
                &dao,
                42,
                9,
                9,
                true,
                false,
                false,
                None,
            )
            .unwrap(),
            vec![ItemCommandExecution::Ignored]
        );
    }
}
