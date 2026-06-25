use crate::dao::{DaoError, InventoryDao};
use crate::game::inventory::{Inventory, InventoryCommandExecution, InventoryRefresh};
use crate::game::item::Item;
use crate::messages::outgoing::{StripInfo, StripItem, StripItemKind};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InventoryCommandExecutor;

impl InventoryCommandExecutor {
    pub fn refresh_inventory(
        inventory_dao: &dyn InventoryDao,
        user_id: i32,
        mode: &str,
    ) -> Result<InventoryCommandExecution, DaoError> {
        let mut items = inventory_dao.inventory_items(user_id)?;
        items.sort_by_key(Item::id);
        let mut inventory = Inventory::with_items(items);

        match inventory.refresh(mode) {
            InventoryRefresh::Page { items, .. } => {
                let strip_items = items.iter().map(strip_item_from_item);
                Ok(InventoryCommandExecution::Refreshed {
                    strip_info: StripInfo::new(strip_items),
                })
            }
            InventoryRefresh::Empty => Ok(InventoryCommandExecution::Empty),
        }
    }
}

fn strip_item_from_item(item: &Item) -> StripItem {
    let definition = item.definition();
    let behaviour = definition.behaviour();
    let custom_data = item.custom_data().unwrap_or_default();
    let kind = if behaviour.is_stuff() {
        StripItemKind::Stuff {
            length: definition.length(),
            width: definition.width(),
            color: definition.color().to_owned(),
        }
    } else if behaviour.is_item() {
        StripItemKind::Item {
            post_it: behaviour.is_post_it(),
        }
    } else {
        StripItemKind::Other
    };

    StripItem::new(
        item.id(),
        definition.sprite(),
        definition.name(),
        custom_data,
        kind,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::{InMemoryInventoryDao, InMemoryItemDao};
    use crate::game::item::ItemDefinition;
    use crate::messages::OutgoingMessage;

    fn definition(id: i32, flags: &str, sprite: &str, name: &str) -> ItemDefinition {
        ItemDefinition::new(id, sprite, "red", 1, 2, 1.0, flags, name, "", "")
    }

    fn item(id: i32, owner_id: i32, definition: ItemDefinition, custom_data: &str) -> Item {
        Item::new(
            id,
            0,
            owner_id,
            "1",
            0,
            0.0,
            0,
            definition,
            "",
            Some(custom_data.to_owned()),
        )
        .unwrap()
    }

    #[test]
    fn refresh_inventory_loads_user_items_into_strip_info() {
        let item_dao = InMemoryItemDao::new();
        item_dao.insert_item(item(1, 7, definition(5, "SF", "chair", "Chair"), "blue"));
        item_dao.insert_item(item(2, 7, definition(6, "IJ", "note", "Post-it"), "1"));
        item_dao.insert_item(item(3, 8, definition(7, "SF", "other", "Other"), "hidden"));
        let inventory_dao = InMemoryInventoryDao::new(item_dao);

        let execution =
            InventoryCommandExecutor::refresh_inventory(&inventory_dao, 7, "new").unwrap();

        let Some(strip_info) = execution.strip_info() else {
            panic!("expected strip info");
        };
        assert_eq!(
            strip_info.compose().get(),
            "#STRIPINFO\rroseau;1;0;S;0;chair;Chair;blue;1;2;red/\rroseau;2;0;I;0;note;Post-it;2;2/##"
        );
    }

    #[test]
    fn refresh_inventory_returns_empty_for_missing_items() {
        let inventory_dao = InMemoryInventoryDao::default();

        assert_eq!(
            InventoryCommandExecutor::refresh_inventory(&inventory_dao, 7, "new").unwrap(),
            InventoryCommandExecution::Empty
        );
    }
}
